use super::{Battle, Event};
use crate::{ability, model::*, pseudorandom::shuffle, status_abnormality};

impl Battle<'_> {
    pub(super) fn eligible(&self, i: usize, slot: usize) -> bool {
        ability::can_activate(
            &self.combatants[i].character.ability_states[slot],
            self.ability(i, slot),
        )
    }

    pub(super) fn slot(&self, i: usize, effect: AbilityEffectID) -> Option<usize> {
        (0..self.combatants[i].character.ability_states.len())
            .find(|&s| self.ability(i, s).effect_id == effect)
    }

    pub(super) fn roll(&mut self, i: usize, slot: usize) -> bool {
        self.eligible(i, slot)
            && self
                .random
                .probability(self.ability(i, slot).activation_rate)
    }

    pub(super) fn mark(&mut self, i: usize, slot: usize) {
        let ability_id = self.ability(i, slot).id;
        ability::mark_activated(&mut self.combatants[i].character.ability_states[slot]);
        self.events.push(Event::Ability {
            actor: i,
            ability_id,
        });
    }

    pub(super) fn activate_effect(&mut self, i: usize, effect: AbilityEffectID) -> Option<usize> {
        let slot = self.slot(i, effect)?;
        if !self.roll(i, slot) {
            return None;
        }
        self.mark(i, slot);
        Some(slot)
    }

    pub(super) fn evaluate_timing(
        &mut self,
        condition: AbilityConditionID,
        timing: Option<AbilityTurnTiming>,
    ) {
        let candidates = (0..self.combatants.len())
            .filter(|&i| {
                self.alive(i)
                    && (0..self.combatants[i].character.ability_states.len()).any(|s| {
                        let c = &self.ability(i, s).activation_condition;
                        self.eligible(i, s)
                            && c.condition_id == condition
                            && timing.is_none_or(|t| {
                                c.turn_timing == t
                                    && ability::every_n_turns(self.turn, c.condition_value)
                            })
                    })
            })
            .collect();
        for i in self.sorted(candidates, true) {
            self.evaluate_character(i, condition, timing);
        }
    }

    pub(super) fn evaluate_character(
        &mut self,
        i: usize,
        condition: AbilityConditionID,
        timing: Option<AbilityTurnTiming>,
    ) {
        for slot in 0..self.combatants[i].character.ability_states.len() {
            let c = &self.ability(i, slot).activation_condition;
            if c.condition_id != condition
                || timing.is_some_and(|t| {
                    c.turn_timing != t || !ability::every_n_turns(self.turn, c.condition_value)
                })
            {
                continue;
            }
            if self.roll(i, slot) {
                self.mark(i, slot);
                self.apply_ability(i, slot);
            }
        }
    }

    pub(super) fn apply_ability(&mut self, actor: usize, slot: usize) {
        let master = self.ability(actor, slot).clone();
        let side = self.combatants[actor].side;
        let mut targets: Vec<_> = (0..self.combatants.len())
            .filter(|&i| {
                if !self.alive(i) {
                    return false;
                }
                match master.target {
                    AbilityTarget::SelfTarget => i == actor,
                    AbilityTarget::AllySingle | AbilityTarget::AllyAll => {
                        self.combatants[i].side == side
                    }
                    AbilityTarget::EnemySingle | AbilityTarget::EnemyAll => {
                        self.combatants[i].side != side
                    }
                    AbilityTarget::None => false,
                }
            })
            .collect();
        targets.sort_by_key(|&i| self.combatants[i].position);
        if matches!(
            master.target,
            AbilityTarget::AllySingle | AbilityTarget::EnemySingle
        ) {
            shuffle(&mut targets, &mut self.random);
            targets.truncate(1);
        }
        for target in targets {
            match &master.effect_data {
                Some(AbilityMasterDataEffectData::StatCorrection(v)) => {
                    let ch = &mut self.combatants[target].character;
                    if master.effect_id == AbilityEffectID::Buff {
                        ch.buff_debuff_effect.attack_buff += v.attack;
                        ch.buff_debuff_effect.defense_buff += v.defense;
                    }
                    if master.effect_id == AbilityEffectID::Debuff {
                        ch.buff_debuff_effect.attack_debuff += v.attack;
                        ch.buff_debuff_effect.defense_debuff += v.defense;
                    }
                    status_abnormality::update_buff_state(ch);
                }
                Some(AbilityMasterDataEffectData::Correction(v))
                    if master.effect_id == AbilityEffectID::Heal =>
                {
                    // Ability recovery = min(max HP * (1 + correction), max HP).
                    let hp = &mut self.combatants[target].character.hp;
                    let amount = (hp.max_hp * (1.0 + v.correction_value)).min(hp.max_hp);
                    hp.heal(amount);
                    self.events.push(Event::Heal {
                        actor,
                        target,
                        amount,
                    });
                }
                _ => {}
            }
        }
    }

    pub(super) fn choose_effect(
        &mut self,
        actor: usize,
        effect: AbilityEffectID,
        same_side: bool,
        predicate: impl Fn(&Self, usize, usize) -> bool,
    ) -> Option<(usize, usize)> {
        let side = self.combatants[actor].side;
        let mut candidates = Vec::new();
        for i in 0..self.combatants.len() {
            if !self.alive(i) || (self.combatants[i].side == side) != same_side {
                continue;
            }
            if let Some(s) = self.slot(i, effect) {
                // Exhaustion is checked before candidate shuffling. Turn flags are checked after selection.
                if self.combatants[i].character.ability_states[s].activation_count
                    < self.ability(i, s).max_activation_count
                    && predicate(self, i, s)
                {
                    candidates.push((i, s));
                }
            }
        }
        candidates.sort_by_key(|&(i, _)| self.combatants[i].position);
        shuffle(&mut candidates, &mut self.random);
        let (i, slot) = candidates.first().copied()?;
        if self.roll(i, slot) {
            self.mark(i, slot);
            Some((i, slot))
        } else {
            None
        }
    }

    pub(super) fn incapacitated(&mut self, i: usize) {
        if !self.alive(i) && !self.death_processed[i] {
            self.death_processed[i] = true;
            self.evaluate_character(i, AbilityConditionID::Incapacitated, None);
        }
    }

    pub(super) fn apply_damage(&mut self, actor: usize, target: usize, damage: f32) {
        let hp = self.combatants[target].character.hp.current_hp;
        let survives = hp > 0.0
            && hp - damage.floor() <= 0.0
            && self
                .activate_effect(target, AbilityEffectID::SurviveAtOneHp)
                .is_some();
        self.combatants[target].character.hp.damage(damage);
        if survives {
            self.combatants[target].character.hp.current_hp = 1.0;
        }
        self.events.push(Event::Damage {
            actor,
            target,
            amount: damage.floor(),
        });
        if self.alive(target) {
            for slot in 0..self.combatants[target].character.ability_states.len() {
                let a = self.ability(target, slot);
                if a.activation_condition.condition_id == AbilityConditionID::HpAtOrBelowThreshold
                    && self.hp_threshold(target, a.activation_condition.condition_value)
                    && self.roll(target, slot)
                {
                    self.mark(target, slot);
                    self.apply_ability(target, slot);
                }
            }
        }
    }

    pub(super) fn hp_threshold(&self, i: usize, percent: u32) -> bool {
        let hp = &self.combatants[i].character.hp;
        // Threshold compares current HP with max HP * (integer percent / 100).
        hp.current_hp <= hp.max_hp * (percent as f32 / 100.0)
    }
}
