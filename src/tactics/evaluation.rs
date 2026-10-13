use super::{Binding, Parameter, Series, applies, integrate, is_active};
use crate::model::*;

#[derive(Clone, Copy, Debug)]
pub struct EffectContext {
    pub binding: Binding,
    pub now: u64,
    pub battle: bool,
    pub interception: bool,
    pub enemy_annihilated: bool,
    pub castle_break: bool,
    pub assault_castle_break: bool,
    pub numbered_castle_break: bool,
    /// Already incremented for the successful sortie being evaluated.
    pub attack_count: u32,
}

impl EffectContext {
    fn triggered(self, trigger: TacticsBattleSpecialTrigger) -> bool {
        match trigger {
            TacticsBattleSpecialTrigger::None => true,
            TacticsBattleSpecialTrigger::Battle => self.battle,
            TacticsBattleSpecialTrigger::Interception => self.interception,
            TacticsBattleSpecialTrigger::EnemyAnnihilated => self.enemy_annihilated,
            TacticsBattleSpecialTrigger::CastleBreak => self.castle_break,
            TacticsBattleSpecialTrigger::AssaultCastleBreak => self.assault_castle_break,
        }
    }
    fn applicable(self, effect: &TacticsActiveEffectState) -> bool {
        is_active(effect, self.now) && applies(effect, self.binding)
    }
}

pub fn modifier(
    effects: &[TacticsActiveEffectState],
    parameter: Parameter,
    context: EffectContext,
) -> f32 {
    let values = effects
        .iter()
        .filter(|e| context.applicable(e))
        .filter_map(|e| {
            if e.effect_id != TacticsEffectID::BattleSpecial {
                return (normal_parameter(e.effect_id) == Some(parameter))
                    .then_some((Series::Effect(e.effect_id), e.effect_value));
            }
            let s = &e.battle_special;
            if !context.triggered(s.trigger) {
                return None;
            }
            if !context.numbered_castle_break
                && ((parameter == Parameter::CastleBreakScore
                    && matches!(
                        s.special_type,
                        TacticsBattleSpecialType::ExDrive | TacticsBattleSpecialType::Slasher
                    ))
                    || parameter == Parameter::CastleBreakScoreLimit)
            {
                return None;
            }
            let value = if parameter == Parameter::BattleScore
                && s.special_type == TacticsBattleSpecialType::Exterlize
            {
                // EXTERLIZE uses the current successful sortie's incremented attack count.
                Some(context.attack_count as f32 * s.parameters.attack_count_score)
            } else {
                parameter.special(s)
            }?;
            Some((Series::Special(s.special_type), value))
        });
    integrate(values, parameter == Parameter::Speed)
}

fn normal_parameter(effect: TacticsEffectID) -> Option<Parameter> {
    match effect {
        TacticsEffectID::AttackCorrection => Some(Parameter::Attack),
        TacticsEffectID::DefenseCorrection => Some(Parameter::Defense),
        TacticsEffectID::SpeedCorrection => Some(Parameter::Speed),
        TacticsEffectID::ScoreCorrection => Some(Parameter::GuildBattleScore),
        TacticsEffectID::ScoreLimitCorrection => Some(Parameter::CastleBreakScoreLimit),
        TacticsEffectID::MaxTpCorrection => Some(Parameter::MaxTp),
        TacticsEffectID::AssaultCastleBreakRateCorrection => {
            Some(Parameter::AssaultCastleBreakRate)
        }
        TacticsEffectID::OpponentSortieSelectionRateCorrection => Some(Parameter::AttackTargetRate),
        // Castle defense is a reference to BattleSpecial.castle_level, not a second correction.
        _ => None,
    }
}

pub fn prevents_assault(effects: &[TacticsActiveEffectState], context: EffectContext) -> bool {
    effects.iter().any(|e| {
        context.applicable(e)
            && e.effect_id == TacticsEffectID::BattleSpecial
            && matches!(
                e.battle_special.special_type,
                TacticsBattleSpecialType::Elysion
                    | TacticsBattleSpecialType::Oratorio
                    | TacticsBattleSpecialType::Berserk
                    | TacticsBattleSpecialType::Panzer
            )
    })
}

/// Selection weights are the additive exception to cross-series multiplication.
pub fn selection_weight(effects: &[TacticsActiveEffectState], context: EffectContext) -> u32 {
    let mut sums = std::collections::BTreeMap::new();
    for e in effects.iter().filter(|e| context.applicable(e)) {
        let (series, value) =
            if e.effect_id == TacticsEffectID::OpponentSortieSelectionRateCorrection {
                (Series::Effect(e.effect_id), e.effect_value)
            } else if e.effect_id == TacticsEffectID::BattleSpecial {
                let s = &e.battle_special;
                let value = match s.special_type {
                    TacticsBattleSpecialType::Provoke => s.parameters.attack_target_rate,
                    TacticsBattleSpecialType::Hide => -s.parameters.attack_target_rate.abs(),
                    TacticsBattleSpecialType::Claustrum => s.parameters.hate,
                    _ => continue,
                };
                (Series::Special(s.special_type), value)
            } else {
                continue;
            };
        *sums.entry(series).or_insert(0.0_f32) += value;
    }
    // Weight = clamp(100 + trunc(100 * sum(series sums)), 1, 1,000,000).
    let correction = 100.0 * sums.values().copied().sum::<f32>();
    let correction = if correction.is_nan() {
        0.0
    } else if correction.is_infinite() {
        99_999.0
    } else {
        correction
    };
    // Widen the addition so a saturated i32 conversion cannot overflow.
    (100_i64 + i64::from(correction as i32)).clamp(1, 1_000_000) as u32
}

/// Fixed resource recovery at successful sortie completion, before consuming COUNT effects.
pub fn recovery(
    effects: &[TacticsActiveEffectState],
    context: EffectContext,
    bp: u16,
    max_bp: u16,
    tp: u8,
    max_tp: u8,
) -> (u16, u8) {
    let mut bp_sum = u64::from(bp);
    let mut tp_sum = u64::from(tp);
    for effect in effects.iter().filter(|e| {
        context.applicable(e)
            && e.effect_id == TacticsEffectID::BattleSpecial
            && context.triggered(e.battle_special.trigger)
    }) {
        bp_sum = bp_sum.saturating_add(u64::from(effect.battle_special.parameters.bp_recovery));
        tp_sum = tp_sum.saturating_add(u64::from(effect.battle_special.parameters.tp_recovery));
    }
    // Final resources = min(current + fixed recovery, configured maximum).
    (
        bp_sum.min(u64::from(max_bp)) as u16,
        tp_sum.min(u64::from(max_tp)) as u8,
    )
}
