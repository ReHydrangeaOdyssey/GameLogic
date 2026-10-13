pub const ABILITY_SLOT_COUNT: usize = 2;
const MAX_HP_THRESHOLD_PERCENT: u32 = 100;
const MIN_HP_THRESHOLD_PERCENT: u32 = 1;
const REQUIRED_DAMAGE_MULTIPLIER: f32 = 2.0;
const CASTLE_BREAK_EFFECT_COUNT: usize = 2;
const CASTLE_BREAK_ATTACK_INDEX: usize = 0;
const CASTLE_BREAK_DAMAGE_INDEX: usize = 1;

mod allowed;
use crate::{Error, model::*, pseudorandom::Random};

pub fn validate(ability: &AbilityMasterData) -> Result<(), Error> {
    use AbilityEffectID as E;
    use AbilityMasterDataEffectData as D;
    crate::rate(ability.activation_rate)?;
    if !allowed::allowed(ability) {
        return Err(Error::InvalidValue("ability effect/condition/target"));
    }
    let valid = match (&ability.effect_data, ability.effect_id) {
        (Some(D::StatCorrection(v)), E::Buff | E::Debuff) => {
            v.attack.is_finite() && v.defense.is_finite()
        }
        (
            Some(D::Correction(v)),
            E::DamageIncrease | E::FixedDamageIncrease | E::Heal | E::CastleBreakDamageIncrease,
        ) => v.correction_value.is_finite(),
        (Some(D::StatusAbnormality(v)), E::Avoidance) => match v.avoidance_type {
            Some(AbilityAvoidanceType::NormalAttack) => v.status.is_none(),
            Some(AbilityAvoidanceType::StatusAbnormality) => matches!(
                v.status,
                Some(
                    StatusAbnormalityID::Silence
                        | StatusAbnormalityID::RangeAttackDisabled
                        | StatusAbnormalityID::Blindness
                )
            ),
            None => false,
        },
        (Some(D::StatusAbnormality(v)), E::StatusAbnormalityAttack) => {
            v.status.is_some() && v.avoidance_type.is_none()
        }
        (
            Some(D::IncapacitatedAllyCountStatCorrection(v)),
            E::IncapacitatedAllyCountStatCorrection,
        ) => valid_entries(&v.entries),
        (
            Some(D::NoParameter(_)),
            E::Counter
            | E::AvoidanceDisable
            | E::CounterDisable
            | E::Cover
            | E::DrawAggro
            | E::Pursuit
            | E::DefenseIgnore
            | E::TargetHpLowPriority
            | E::TargetDefenseDownPriority
            | E::DrawAggroIgnore
            | E::AvoidanceCounter
            | E::SurviveAtOneHp,
        ) => true,
        _ => false,
    };
    if !valid {
        return Err(Error::InvalidValue("ability effect data"));
    }
    let c = &ability.activation_condition;
    if c.condition_id == AbilityConditionID::EveryNTurns && c.condition_value == 0 {
        return Err(Error::InvalidValue("ability interval"));
    }
    if matches!(
        c.condition_id,
        AbilityConditionID::HpAtOrBelowThreshold
            | AbilityConditionID::AllyHpAtOrBelowThresholdAttacked
            | AbilityConditionID::TargetHpAtOrBelowThresholdNormalAttack
    ) && !(MIN_HP_THRESHOLD_PERCENT..=MAX_HP_THRESHOLD_PERCENT).contains(&c.condition_value)
    {
        return Err(Error::InvalidValue("ability HP threshold"));
    }
    if (ability.effect_id == E::CastleBreakDamageIncrease
        || (ability.effect_id == E::DamageIncrease
            && c.condition_id == AbilityConditionID::SingleTargetNormalAttack))
        && correction(ability) != REQUIRED_DAMAGE_MULTIPLIER
    {
        return Err(Error::InvalidValue("ability damage multiplier"));
    }
    Ok(())
}

fn valid_entries(entries: &[AbilityIncapacitatedAllyCountStatCorrectionEntry]) -> bool {
    if entries.len() != crate::formation::MAX_PARTY_SIZE {
        return false;
    }
    let mut entries: Vec<_> = entries.iter().collect();
    entries.sort_by_key(|e| e.incapacitated_ally_count);
    entries.iter().enumerate().all(|(i, e)| {
        e.incapacitated_ally_count == i as u32 && e.attack.is_finite() && e.defense.is_finite()
    }) && entries
        .windows(2)
        .all(|pair| pair[0].attack <= pair[1].attack && pair[0].defense <= pair[1].defense)
}

pub fn every_n_turns(current_turn: u32, interval: u32) -> bool {
    // Multiples of N are eligible only after turn zero.
    interval > 0 && current_turn > 0 && current_turn.is_multiple_of(interval)
}

pub fn validate_equipment(
    character: &CharacterMasterData,
    abilities: &[&AbilityMasterData],
) -> Result<(), Error> {
    if abilities.len() > ABILITY_SLOT_COUNT {
        return Err(Error::InvalidEquipment);
    }
    for (i, a) in abilities.iter().enumerate() {
        if !character.ability_ids.contains(&a.id)
            || !a.allowed_attributes.contains(&character.attribute)
            || abilities[..i]
                .iter()
                .any(|other| other.id == a.id || other.effect_id == a.effect_id)
        {
            return Err(Error::InvalidEquipment);
        }
    }
    Ok(())
}

pub fn can_activate(state: &AbilityBattleState, master: &AbilityMasterData) -> bool {
    state.activation_count < master.max_activation_count && !state.activated_this_turn
}

pub(crate) fn mark_activated(state: &mut AbilityBattleState) {
    state.activation_count = state.activation_count.saturating_add(1);
    state.activated_this_turn = true;
}

pub fn correction(ability: &AbilityMasterData) -> f32 {
    if let Some(AbilityMasterDataEffectData::Correction(data)) = &ability.effect_data {
        data.correction_value
    } else {
        0.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CastleBreakCorrection {
    pub attack: f32,
    pub damage: f32,
}

/// Independent of battle counters; the caller invokes this once per selected character and CB.
pub fn castle_break(
    abilities: &[&AbilityMasterData],
    random: &mut Random,
) -> Result<CastleBreakCorrection, Error> {
    let mut result = CastleBreakCorrection::default();
    let mut checked = [false; CASTLE_BREAK_EFFECT_COUNT];
    for a in abilities {
        validate(a)?;
        if a.activation_condition.condition_id != AbilityConditionID::CastleBreak {
            continue;
        }
        let index = match a.effect_id {
            AbilityEffectID::Buff => CASTLE_BREAK_ATTACK_INDEX,
            AbilityEffectID::CastleBreakDamageIncrease => CASTLE_BREAK_DAMAGE_INDEX,
            _ => continue,
        };
        if checked[index] {
            continue;
        }
        checked[index] = true;
        if !random.probability(a.activation_rate) {
            continue;
        }
        if let Some(AbilityMasterDataEffectData::StatCorrection(v)) = &a.effect_data {
            result.attack = v.attack;
        }
        if index == CASTLE_BREAK_DAMAGE_INDEX {
            // The CB formula expects a correction, so convert multiplier m to m - 1.
            result.damage = correction(a) - 1.0;
        }
    }
    Ok(result)
}
