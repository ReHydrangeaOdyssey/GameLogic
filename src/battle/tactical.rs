use super::{Battle, Mode};
use crate::{
    model::*,
    tactics::{self, Binding, Series},
};

#[derive(Clone, Copy)]
pub(super) enum Stat {
    Attack,
    Defense,
    Speed,
    SkillRate,
}

impl Battle<'_> {
    pub(super) fn applies(&self, i: usize, effect: &TacticsActiveEffectState) -> bool {
        if self.mode != Mode::GuildBattle || !tactics::is_active(effect, self.now) {
            return false;
        }
        let c = &self.combatants[i];
        let source_side = self
            .combatants
            .iter()
            .find(|v| v.character.owner_player_id == effect.source_player_id)
            .map(|v| v.side);
        let opponent = source_side
            .and_then(|side| self.combatants.iter().find(|v| v.side != side))
            .map(|v| v.character.owner_player_id);
        tactics::applies(
            effect,
            Binding {
                player_id: c.character.owner_player_id,
                guild_id: c.guild_id,
                source_opponent_id: opponent,
                castle_break: false,
            },
        )
    }

    pub(super) fn trigger_active(&self, i: usize, trigger: TacticsBattleSpecialTrigger) -> bool {
        matches!(
            trigger,
            TacticsBattleSpecialTrigger::None | TacticsBattleSpecialTrigger::Battle
        ) || (trigger == TacticsBattleSpecialTrigger::Interception && self.combatants[i].side == 1)
    }

    pub(super) fn tactics_correction(&self, i: usize, stat: Stat) -> f32 {
        let normal = match stat {
            Stat::Attack => Some(TacticsEffectID::AttackCorrection),
            Stat::Defense => Some(TacticsEffectID::DefenseCorrection),
            Stat::Speed => Some(TacticsEffectID::SpeedCorrection),
            Stat::SkillRate => None,
        };
        let values = self
            .effects
            .iter()
            .filter(|e| self.applies(i, e))
            .filter_map(|e| {
                if Some(e.effect_id) == normal {
                    return Some((Series::Effect(e.effect_id), e.effect_value));
                }
                if e.effect_id == TacticsEffectID::BattleSpecial
                    && self.trigger_active(i, e.battle_special.trigger)
                {
                    let parameter = match stat {
                        Stat::Attack => tactics::Parameter::Attack,
                        Stat::Defense => tactics::Parameter::Defense,
                        Stat::Speed => tactics::Parameter::Speed,
                        Stat::SkillRate => tactics::Parameter::SkillActivationRate,
                    };
                    return parameter
                        .special(&e.battle_special)
                        .map(|v| (Series::Special(e.battle_special.special_type), v));
                }
                None
            });
        tactics::integrate(values, matches!(stat, Stat::Speed))
    }

    pub(super) fn has_special(&self, i: usize, special: TacticsBattleSpecialType) -> bool {
        self.effects.iter().any(|e| {
            e.effect_id == TacticsEffectID::BattleSpecial
                && e.battle_special.special_type == special
                && self.applies(i, e)
                && self.trigger_active(i, e.battle_special.trigger)
        })
    }

    pub(super) fn erase(&mut self, target: usize) -> bool {
        let indices: Vec<_> = self
            .effects
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                e.effect_id == TacticsEffectID::BattleSpecial
                    && e.battle_special.special_type == TacticsBattleSpecialType::Erase
                    && !e.erase_consumed
                    && self.applies(target, e)
                    && self.trigger_active(target, e.battle_special.trigger)
            })
            .map(|(i, _)| i)
            .collect();
        for &i in &indices {
            self.effects[i].erase_consumed = true;
        }
        !indices.is_empty()
    }

    pub(super) fn stat(&self, i: usize, attack: bool) -> f32 {
        let c = &self.combatants[i];
        let buffs = &c.character.buff_debuff_effect;
        let (base, formation, buff) = if attack {
            (
                c.character.attack,
                c.formation.attack,
                buffs.attack_buff - buffs.attack_debuff,
            )
        } else {
            (
                c.character.defense,
                c.formation.defense,
                buffs.defense_buff - buffs.defense_debuff,
            )
        };
        let dead = self
            .combatants
            .iter()
            .enumerate()
            .filter(|(index, v)| {
                *index != i && v.side == c.side && v.character.hp.current_hp == 0.0
            })
            .count();
        let mut ally_correction = 0.0;
        for (slot, state) in c.character.ability_states.iter().enumerate() {
            if state.activation_count == 0 {
                continue;
            }
            if let Some(AbilityMasterDataEffectData::IncapacitatedAllyCountStatCorrection(data)) =
                &self.ability(i, slot).effect_data
                && let Some(entry) = data
                    .entries
                    .iter()
                    .find(|e| e.incapacitated_ally_count == dead as u32)
            {
                ally_correction += if attack { entry.attack } else { entry.defense };
            }
        }
        let tactics = self.tactics_correction(i, if attack { Stat::Attack } else { Stat::Defense });
        // Stat = max(base * (1+formation) * (1+buff-debuff) * (1+dead ally correction) * (1+tactics), 0).
        (base * (1.0 + formation) * (1.0 + buff) * (1.0 + ally_correction) * (1.0 + tactics))
            .max(0.0)
    }
}
