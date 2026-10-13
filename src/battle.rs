mod abilities;
mod actions;
mod order;
mod tactical;

use crate::{
    Error,
    formation::{Correction, Position},
    master_data::MasterData,
    model::*,
    pseudorandom::Random,
    status_abnormality,
};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Arena,
    GuildBattle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Combatant {
    pub character: CharacterBattle,
    /// 0: requesting/attacking party, 1: defending party.
    pub side: u8,
    pub guild_id: u64,
    pub position: Position,
    pub formation: Correction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Action {
        actor: usize,
    },
    Skill {
        actor: usize,
        skill_id: u32,
    },
    Ability {
        actor: usize,
        ability_id: u32,
    },
    Damage {
        actor: usize,
        target: usize,
        amount: f32,
    },
    Heal {
        actor: usize,
        target: usize,
        amount: f32,
    },
    Status {
        actor: usize,
        target: usize,
        status: StatusAbnormalityID,
    },
    Avoided {
        actor: usize,
        target: usize,
    },
    ForcedTurn,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    pub number: u32,
    pub actor: Option<usize>,
    pub events: Vec<Event>,
}

/// A step-based simulation. Victory and turn-limit policies belong to the caller.
#[derive(Clone, Debug)]
pub struct Battle<'a> {
    master: &'a MasterData,
    pub(super) mode: Mode,
    pub(super) combatants: Vec<Combatant>,
    pub(super) random: Random,
    pub(super) turn: u32,
    pub(super) attack_queue: VecDeque<usize>,
    pub(super) wait_queue: VecDeque<usize>,
    pub(super) wait: Vec<u32>,
    pub(super) effects: Vec<TacticsActiveEffectState>,
    pub(super) events: Vec<Event>,
    pub(super) counter_used: bool,
    pub(super) death_processed: Vec<bool>,
    pub(super) now: u64,
}

impl<'a> Battle<'a> {
    pub fn new(
        master: &'a MasterData,
        mode: Mode,
        combatants: Vec<Combatant>,
        seed: u64,
        effects: Vec<TacticsActiveEffectState>,
        now: u64,
    ) -> Result<Self, Error> {
        validate_combatants(master, &combatants)?;
        for effect in &effects {
            crate::tactics::validate_active(effect, master.tactics(effect.tactics_id)?)?;
            let speed = if effect.effect_id == TacticsEffectID::SpeedCorrection {
                effect.effect_value
            } else {
                effect.battle_special.parameters.speed
            };
            if speed.fract() != 0.0 {
                return Err(Error::InvalidValue("tactics speed level"));
            }
        }
        let count = combatants.len();
        let mut battle = Self {
            master,
            mode,
            combatants,
            random: Random::new(seed),
            turn: 0,
            attack_queue: VecDeque::new(),
            wait_queue: VecDeque::new(),
            wait: vec![0; count],
            effects,
            events: Vec::new(),
            counter_used: false,
            death_processed: vec![false; count],
            now,
        };
        for c in &mut battle.combatants {
            c.character.skill_state.activation_count = 0;
            for state in &mut c.character.ability_states {
                state.activation_count = 0;
                state.activated_this_turn = false;
            }
            status_abnormality::update_buff_state(&mut c.character);
        }
        crate::tactics::expire(&mut battle.effects, now);
        battle.evaluate_timing(AbilityConditionID::BattleStart, None);
        battle.initial_order();
        Ok(battle)
    }

    pub fn combatants(&self) -> &[Combatant] {
        &self.combatants
    }
    pub fn turn_count(&self) -> u32 {
        self.turn
    }
    pub fn random_state(&self) -> u64 {
        self.random.state()
    }
    pub fn tactics_effects(&self) -> &[TacticsActiveEffectState] {
        &self.effects
    }
    pub fn pending_events(&self) -> &[Event] {
        &self.events
    }
    pub fn annihilated(&self, side: u8) -> bool {
        !self
            .combatants
            .iter()
            .any(|c| c.side == side && c.character.hp.current_hp > 0.0)
    }

    pub fn step(&mut self) -> Result<Option<Turn>, Error> {
        if self.annihilated(0) || self.annihilated(1) {
            return Ok(None);
        }
        let next_turn = self.turn.checked_add(1).ok_or(Error::ArithmeticOverflow)?;
        // Defer next-turn scheduling until the caller has evaluated victory and turn limits.
        if self.turn > 0 && (0..self.combatants.len()).any(|i| self.actable(i)) {
            self.update_wait();
            self.refill();
        }
        self.counter_used = false;
        for c in &mut self.combatants {
            for state in &mut c.character.ability_states {
                state.activated_this_turn = false;
            }
        }
        self.filter_queues();
        let actor = if (0..self.combatants.len()).any(|i| self.actable(i)) {
            while self.attack_queue.is_empty() {
                self.update_wait();
                self.refill();
            }
            self.attack_queue.pop_front()
        } else {
            None
        };
        if let Some(actor) = actor {
            self.events.push(Event::Action { actor });
            self.evaluate_timing(
                AbilityConditionID::EveryNTurns,
                Some(AbilityTurnTiming::TurnStart),
            );
            self.evaluate_timing(
                AbilityConditionID::EveryNTurns,
                Some(AbilityTurnTiming::BeforeAction),
            );
            self.action(actor);
            self.evaluate_timing(
                AbilityConditionID::EveryNTurns,
                Some(AbilityTurnTiming::AfterAction),
            );
        } else {
            self.events.push(Event::ForcedTurn);
        }
        self.end_statuses();
        self.evaluate_timing(
            AbilityConditionID::EveryNTurns,
            Some(AbilityTurnTiming::TurnEnd),
        );
        if let Some(actor) = actor {
            self.after_action_wait(actor);
        }
        self.turn = next_turn;
        let result = Turn {
            number: next_turn,
            actor,
            events: std::mem::take(&mut self.events),
        };
        Ok(Some(result))
    }

    pub(super) fn alive(&self, i: usize) -> bool {
        self.combatants[i].character.hp.current_hp > 0.0
    }
    pub(super) fn actable(&self, i: usize) -> bool {
        self.alive(i)
            && !status_abnormality::contains(
                &self.combatants[i].character,
                StatusAbnormalityID::Coma,
            )
    }
    pub(super) fn skill(&self, i: usize) -> &SkillMasterData {
        self.master
            .skill(self.combatants[i].character.skill_state.skill_id)
            .expect("validated skill")
    }
    pub(super) fn ability(&self, i: usize, slot: usize) -> &AbilityMasterData {
        self.master
            .ability(self.combatants[i].character.ability_states[slot].ability_id)
            .expect("validated ability")
    }

    fn end_statuses(&mut self) {
        let mut order: Vec<_> = (0..self.combatants.len()).collect();
        order.sort_by_key(|&i| {
            let c = &self.combatants[i];
            (c.position, c.character.owner_player_id, c.side)
        });
        for i in order {
            if !self.alive(i) {
                continue;
            }
            if status_abnormality::end_turn(&mut self.combatants[i].character, &mut self.random) {
                self.wait[i] += self.speed(i);
            }
            // Poison deaths do not activate incapacitation abilities.
            if !self.alive(i) {
                self.death_processed[i] = true;
            }
        }
    }
}

fn validate_combatants(master: &MasterData, combatants: &[Combatant]) -> Result<(), Error> {
    for side in 0..2 {
        let count = combatants.iter().filter(|c| c.side == side).count();
        if !(1..=5).contains(&count) {
            return Err(Error::InvalidFormation);
        }
    }
    for (i, c) in combatants.iter().enumerate() {
        if c.side > 1
            || combatants[..i].iter().any(|p| {
                p.side == c.side && (p.position == c.position || p.character.id == c.character.id)
            })
        {
            return Err(Error::InvalidFormation);
        }
        let ch = &c.character;
        let base = master.character(ch.id)?;
        if ch.owner_player_id == u64::MAX || ch.bp > 99 || ch.attribute != base.attribute {
            return Err(Error::InvalidValue("battle character"));
        }
        crate::nonnegative(ch.hp.max_hp, "max_hp")?;
        crate::nonnegative(ch.hp.current_hp, "current_hp")?;
        crate::nonnegative(ch.attack, "attack")?;
        crate::nonnegative(ch.defense, "defense")?;
        if ch.hp.max_hp == 0.0 || ch.hp.current_hp > ch.hp.max_hp {
            return Err(Error::InvalidValue("HP"));
        }
        master.skill(ch.skill_state.skill_id)?;
        let abilities = ch
            .ability_states
            .iter()
            .map(|a| master.ability(a.ability_id))
            .collect::<Result<Vec<_>, _>>()?;
        crate::ability::validate_equipment(base, &abilities)?;
        for value in [
            c.formation.attack,
            c.formation.defense,
            c.formation.speed,
            c.formation.skill_activation_rate,
        ] {
            if !value.is_finite() {
                return Err(Error::InvalidValue("formation correction"));
            }
        }
        if c.formation.speed.fract() != 0.0 {
            return Err(Error::InvalidValue("formation speed"));
        }
        let b = &ch.buff_debuff_effect;
        for value in [
            b.attack_buff,
            b.attack_debuff,
            b.defense_buff,
            b.defense_debuff,
        ] {
            crate::nonnegative(value, "buff/debuff")?;
        }
        let mut ids = std::collections::BTreeSet::new();
        if ch
            .status_abnormalities
            .iter()
            .any(|s| s.elapsed_turns == 0 || s.poison_cycle_turns > 2 || !ids.insert(s.status_id))
        {
            return Err(Error::InvalidValue("status state"));
        }
    }
    Ok(())
}
