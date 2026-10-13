use crate::{Error, model::TacticsBattleSpecialParameters};
pub(super) fn add_scaled(
    base: &mut TacticsBattleSpecialParameters,
    increase: &TacticsBattleSpecialParameters,
    steps: u32,
) -> Result<(), Error> {
    // Each parameter = base + (stage - 1) * per-stage increase.
    base.attack += steps as f32 * increase.attack;
    if !base.attack.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.defense += steps as f32 * increase.defense;
    if !base.defense.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.speed += steps as f32 * increase.speed;
    if !base.speed.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.skill_activation_rate += steps as f32 * increase.skill_activation_rate;
    if !base.skill_activation_rate.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.max_tp += steps as f32 * increase.max_tp;
    if !base.max_tp.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.battle_score += steps as f32 * increase.battle_score;
    if !base.battle_score.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.guild_battle_score += steps as f32 * increase.guild_battle_score;
    if !base.guild_battle_score.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.castle_break_score += steps as f32 * increase.castle_break_score;
    if !base.castle_break_score.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.assault_castle_break_rate += steps as f32 * increase.assault_castle_break_rate;
    if !base.assault_castle_break_rate.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.assault_castle_break_score += steps as f32 * increase.assault_castle_break_score;
    if !base.assault_castle_break_score.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.hate += steps as f32 * increase.hate;
    if !base.hate.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.castle_level = base
        .castle_level
        .checked_add(
            increase
                .castle_level
                .checked_mul(steps as i32)
                .ok_or(Error::ArithmeticOverflow)?,
        )
        .ok_or(Error::ArithmeticOverflow)?;
    base.bp_recovery = base
        .bp_recovery
        .checked_add(
            increase
                .bp_recovery
                .checked_mul(steps)
                .ok_or(Error::ArithmeticOverflow)?,
        )
        .ok_or(Error::ArithmeticOverflow)?;
    base.tp_recovery = base
        .tp_recovery
        .checked_add(
            increase
                .tp_recovery
                .checked_mul(steps)
                .ok_or(Error::ArithmeticOverflow)?,
        )
        .ok_or(Error::ArithmeticOverflow)?;
    base.attack_count_score += steps as f32 * increase.attack_count_score;
    if !base.attack_count_score.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.castle_break_score_limit += steps as f32 * increase.castle_break_score_limit;
    if !base.castle_break_score_limit.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.hp_recovery_value += steps as f32 * increase.hp_recovery_value;
    if !base.hp_recovery_value.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.revive_rate += steps as f32 * increase.revive_rate;
    if !base.revive_rate.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    base.attack_target_rate += steps as f32 * increase.attack_target_rate;
    if !base.attack_target_rate.is_finite() {
        return Err(Error::ArithmeticOverflow);
    }
    Ok(())
}
pub(super) fn finite(p: &TacticsBattleSpecialParameters) -> bool {
    p.attack.is_finite()
        && p.defense.is_finite()
        && p.speed.is_finite()
        && p.skill_activation_rate.is_finite()
        && p.max_tp.is_finite()
        && p.battle_score.is_finite()
        && p.guild_battle_score.is_finite()
        && p.castle_break_score.is_finite()
        && p.assault_castle_break_rate.is_finite()
        && p.assault_castle_break_score.is_finite()
        && p.hate.is_finite()
        && p.attack_count_score.is_finite()
        && p.castle_break_score_limit.is_finite()
        && p.hp_recovery_value.is_finite()
        && p.revive_rate.is_finite()
        && p.attack_target_rate.is_finite()
}
