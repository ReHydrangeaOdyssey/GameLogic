const MAX_TACTICS_STAGE: u32 = 5;
const MAX_TACTICS_USES: u32 = 5;
const BASE_TACTICS_STAGE: u32 = 1;
const MICROSECONDS_PER_SECOND: u64 = 1_000_000;

mod evaluation;
mod parameter;
mod parameters;
mod usage;
mod validation;
use crate::{Error, model::*, pseudorandom::Random};
pub use evaluation::{EffectContext, modifier, prevents_assault, recovery, selection_weight};
pub use parameter::Parameter;
use std::collections::BTreeMap;
pub use usage::{PartyResources, UseContext, UseResult, apply_immediate, use_tactics};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Series {
    Effect(TacticsEffectID),
    Special(TacticsBattleSpecialType),
}

pub fn integrate(values: impl IntoIterator<Item = (Series, f32)>, speed: bool) -> f32 {
    let mut sums = BTreeMap::new();
    let mut total = 0.0;
    for (series, value) in values {
        total += value;
        *sums.entry(series).or_insert(0.0) += value;
    }
    if speed {
        return total;
    }
    // Same-series sum, then product of distinct series; no active effect means zero correction.
    if sums.is_empty() {
        0.0
    } else {
        sums.values().copied().product()
    }
}

pub fn validate(master: &TacticsMasterData) -> Result<(), Error> {
    use TacticsEffectDataEffectData as D;
    if master.effects.is_empty() {
        return Err(Error::InvalidValue("tactics effects"));
    }
    let mut resurrection = false;
    for effect in &master.effects {
        let valid = match (&effect.effect_data, effect.effect_id) {
            (Some(D::UintValue(_)), TacticsEffectID::BpRecovery) => {
                master.end_type == TacticsEndType::OnActivation
            }
            (Some(D::HpRecovery(v)), TacticsEffectID::HpRecovery) => {
                master.end_type == TacticsEndType::OnActivation
                    && crate::rate(v.recovery_rate).is_ok()
            }
            (Some(D::BattleSpecial(v)), TacticsEffectID::BattleSpecial) => {
                resurrection |= v.special_type == TacticsBattleSpecialType::Resurrection;
                parameters::finite(&v.parameters)
                    && validation::valid_special(v, effect.target, master.end_type)
                    && crate::rate(v.parameters.revive_rate).is_ok()
            }
            (Some(D::CorrectionValue(v)), id) => {
                v.is_finite()
                    && !matches!(
                        id,
                        TacticsEffectID::BpRecovery
                            | TacticsEffectID::HpRecovery
                            | TacticsEffectID::BattleSpecial
                    )
            }
            _ => false,
        };
        if !valid {
            return Err(Error::InvalidValue("tactics effect data"));
        }
    }
    if resurrection != (master.use_condition == TacticsUseCondition::AllAnnihilated) {
        return Err(Error::InvalidValue("tactics use condition"));
    }
    if (master.end_type == TacticsEndType::Duration && master.duration_seconds == 0)
        || (master.end_type == TacticsEndType::Count && master.effect_count == 0)
    {
        return Err(Error::InvalidValue("tactics end condition"));
    }
    let mut indices = std::collections::BTreeSet::new();
    for stage in &master.stage_effects {
        let effect = master
            .effects
            .get(stage.effect_index as usize)
            .ok_or(Error::InvalidValue("tactics effect index"))?;
        if stage.effect_id != effect.effect_id
            || stage.target != effect.target
            || !indices.insert(stage.effect_index)
        {
            return Err(Error::InvalidValue("tactics stage mapping"));
        }
        let mut effect = effect.clone();
        increase(&mut effect, stage, 1)?;
    }
    Ok(())
}

fn increase(
    effect: &mut TacticsEffectData,
    stage: &TacticsStageEffectData,
    steps: u32,
) -> Result<(), Error> {
    use TacticsEffectDataEffectData as D;
    use TacticsStageEffectDataIncreaseData as I;
    // Final effect = base + (stage - 1) * increase, preserving f32 order.
    match (&mut effect.effect_data, &stage.increase_data) {
        (Some(D::CorrectionValue(v)), Some(I::IncreaseValue(i))) => *v += steps as f32 * i,
        (Some(D::UintValue(v)), Some(I::IncreaseUintValue(i))) => {
            *v = v
                .checked_add(steps.checked_mul(*i).ok_or(Error::ArithmeticOverflow)?)
                .ok_or(Error::ArithmeticOverflow)?
        }
        (Some(D::HpRecovery(v)), Some(I::IncreaseValue(i)))
            if v.recovery_type == TacticsHpRecoveryType::PositiveHpRate =>
        {
            v.recovery_rate += steps as f32 * i
        }
        (Some(D::BattleSpecial(v)), Some(I::BattleSpecialIncrease(i))) => {
            parameters::add_scaled(&mut v.parameters, i, steps)?
        }
        _ => return Err(Error::InvalidValue("tactics stage data")),
    }
    Ok(())
}

pub fn stage_effects(
    master: &TacticsMasterData,
    owned_count: u32,
) -> Result<Vec<TacticsEffectData>, Error> {
    validate(master)?;
    if owned_count == 0 {
        return Err(Error::TacticsUnavailable);
    }
    let steps = owned_count.min(MAX_TACTICS_STAGE) - BASE_TACTICS_STAGE;
    let mut effects = master.effects.clone();
    for stage in &master.stage_effects {
        increase(&mut effects[stage.effect_index as usize], stage, steps)?;
    }
    let mut scaled = master.clone();
    scaled.effects = effects.clone();
    scaled.stage_effects.clear();
    validate(&scaled)?;
    Ok(effects)
}

pub fn initial_state(master: &TacticsMasterData, owned_count: u32) -> TacticsBattleState {
    TacticsBattleState {
        available_uses: owned_count.min(MAX_TACTICS_USES),
        tp_cost: master.tp_cost,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Source {
    pub player_id: u64,
    pub guild_id: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub player_id: u64,
    pub guild_id: u64,
    /// The opponent of the effect's source in the current battle, not at activation time.
    pub source_opponent_id: Option<u64>,
    pub castle_break: bool,
}

pub fn applies(effect: &TacticsActiveEffectState, binding: Binding) -> bool {
    match effect.target {
        TacticsTarget::SelfParty => effect.source_player_id == binding.player_id,
        TacticsTarget::OpponentParty => binding.source_opponent_id == Some(binding.player_id),
        TacticsTarget::AllyGuild => effect.source_guild_id == binding.guild_id,
        TacticsTarget::EnemyGuild => effect.source_guild_id != binding.guild_id,
        TacticsTarget::CastleBreak => binding.castle_break,
    }
}

pub fn is_active(effect: &TacticsActiveEffectState, now: u64) -> bool {
    match effect.end_type {
        TacticsEndType::Duration => now < effect.expires_at,
        TacticsEndType::Count => effect.remaining_count > 0,
        TacticsEndType::OnActivation => false,
    }
}

pub fn validate_active(
    effect: &TacticsActiveEffectState,
    master: &TacticsMasterData,
) -> Result<(), Error> {
    if effect.tactics_id != master.id
        || effect.end_type != master.end_type
        || !effect.effect_value.is_finite()
        || !parameters::finite(&effect.battle_special.parameters)
        || effect.source_player_id == u64::MAX
    {
        return Err(Error::InvalidValue("active tactics effect"));
    }
    let matches = master.effects.iter().any(|entry| {
        entry.effect_id == effect.effect_id && entry.target == effect.target
            && (entry.effect_id != TacticsEffectID::BattleSpecial
                || matches!(&entry.effect_data, Some(TacticsEffectDataEffectData::BattleSpecial(v)) if v.special_type == effect.battle_special.special_type && v.trigger == effect.battle_special.trigger))
    });
    if !matches {
        return Err(Error::InvalidValue("active tactics mapping"));
    }
    if effect.effect_id == TacticsEffectID::BattleSpecial
        && !validation::valid_special(&effect.battle_special, effect.target, effect.end_type)
    {
        return Err(Error::InvalidValue("active battle special"));
    }
    Ok(())
}

pub fn expire(effects: &mut Vec<TacticsActiveEffectState>, now: u64) {
    effects.retain(|e| is_active(e, now));
}

/// Call after applying this event's effect, so the final remaining use still applies.
pub fn consume(effects: &mut Vec<TacticsActiveEffectState>, event: TacticsCountConsumeTrigger) {
    for effect in effects.iter_mut() {
        if effect.end_type == TacticsEndType::Count && effect.count_consume_trigger == event {
            effect.remaining_count = effect.remaining_count.saturating_sub(1);
        }
    }
    effects.retain(|e| e.end_type != TacticsEndType::Count || e.remaining_count > 0);
}

pub fn dedicated_seed(effects: &[TacticsEffectData], random: &mut Random) -> u64 {
    if effects.iter().any(|e| matches!(&e.effect_data, Some(TacticsEffectDataEffectData::BattleSpecial(v)) if matches!(v.special_type, TacticsBattleSpecialType::Revive | TacticsBattleSpecialType::Resurrection))) {
        u64::from(random.next_u32())
    } else { 0 }
}

pub fn recover_hp(hp: &mut HitPoints, recovery: &TacticsHpRecoveryData) {
    match recovery.recovery_type {
        TacticsHpRecoveryType::IncapacitatedFull if hp.current_hp == 0.0 => {
            hp.current_hp = hp.max_hp
        }
        TacticsHpRecoveryType::PositiveHpRate if hp.current_hp > 0.0 => {
            hp.heal(hp.max_hp * recovery.recovery_rate)
        }
        _ => {}
    }
}

/// Targets are formation selection IDs with HP; ordering does not depend on input order.
pub fn revive(targets: &mut [(u8, HitPoints)], rate: f32, seed: u64) -> Result<(), Error> {
    crate::rate(rate)?;
    let mut order: Vec<_> = (0..targets.len()).collect();
    order.sort_by_key(|&i| targets[i].0);
    let mut random = Random::new(seed);
    for i in order {
        let hp = &mut targets[i].1;
        if hp.current_hp == 0.0 && random.probability(rate) {
            hp.current_hp = hp.max_hp;
        }
    }
    Ok(())
}

/// Convert scaled effects to runtime records. Repeated uses preserve expiry, count and ERASE state.
pub fn activate(
    master: &TacticsMasterData,
    effects: &[TacticsEffectData],
    active: &mut Vec<TacticsActiveEffectState>,
    source: Source,
    now: u64,
) -> Result<(), Error> {
    if master.end_type == TacticsEndType::OnActivation {
        return Ok(());
    }
    let expires_at = if master.end_type == TacticsEndType::Duration {
        // Absolute microseconds = supplied now + duration seconds * 1,000,000.
        now.checked_add(u64::from(master.duration_seconds) * MICROSECONDS_PER_SECOND)
            .ok_or(Error::ArithmeticOverflow)?
    } else {
        0
    };
    let mut next = active.clone();
    expire(&mut next, now);
    let existing: Vec<_> = next
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            e.tactics_id == master.id
                && e.source_player_id == source.player_id
                && e.source_guild_id == source.guild_id
        })
        .map(|(i, _)| i)
        .collect();
    for (index, effect) in effects.iter().enumerate() {
        let (value, special) = match &effect.effect_data {
            Some(TacticsEffectDataEffectData::CorrectionValue(v)) => {
                (*v, TacticsBattleSpecialData::default())
            }
            Some(TacticsEffectDataEffectData::BattleSpecial(v)) => (0.0, v.clone()),
            _ => return Err(Error::InvalidValue("persistent tactics effect")),
        };
        if let Some(&i) = existing.get(index) {
            next[i].effect_value += value;
            parameters::add_scaled(
                &mut next[i].battle_special.parameters,
                &special.parameters,
                1,
            )?;
        } else {
            next.push(TacticsActiveEffectState {
                tactics_id: master.id,
                effect_id: effect.effect_id,
                target: effect.target,
                effect_value: value,
                expires_at,
                remaining_count: master.effect_count,
                battle_special: special,
                count_consume_trigger: master.count_consume_trigger,
                end_type: master.end_type,
                erase_consumed: false,
                source_player_id: source.player_id,
                source_guild_id: source.guild_id,
            });
        }
    }
    *active = next;
    Ok(())
}
