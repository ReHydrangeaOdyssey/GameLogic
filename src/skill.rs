const DEFENSE_DAMAGE_DIVISOR: f32 = 3.0;
const LOW_HP_TARGET_RATE: f32 = 0.25;
const BUFF_DEBUFF_ACTIVATION_LIMIT: u32 = 1;

use crate::{
    Error,
    formation::{Position, best_column},
    model::*,
    pseudorandom::{Random, shuffle},
    status_abnormality,
};

pub const BASE_ACTIVATION_RATE: f32 = 0.2;
pub const MIN_DAMAGE: f32 = 250.0;
pub const MAX_NORMAL_DAMAGE: f32 = 99_999.0;

pub fn activation_rate(skill: &SkillMasterData, formation: f32, tactics: f32) -> f32 {
    // Activation = clamp(0.2 + skill + formation + tactics, 0, 1).
    (BASE_ACTIVATION_RATE + skill.activation_rate + formation + tactics).clamp(0.0, 1.0)
}

pub fn normal_damage(attack: f32, defense: f32, multiplier: f32, random: &mut Random) -> f32 {
    // Normal: min(max(attack - defense / 3, 250) * random * ability multiplier, 99999).
    ((attack - defense / DEFENSE_DAMAGE_DIVISOR).max(MIN_DAMAGE)
        * random.damage_multiplier()
        * multiplier)
        .min(MAX_NORMAL_DAMAGE)
}

pub fn attack_damage(
    skill: &SkillMasterData,
    attack: f32,
    defense: f32,
    fixed_increase: f32,
    random: &mut Random,
) -> f32 {
    match skill.damage_value_type {
        // Fixed attacks consume no damage random value.
        SkillDamageValueType::Fixed => {
            (skill.correction_value + fixed_increase).clamp(MIN_DAMAGE, MAX_NORMAL_DAMAGE)
        }
        // Rate: max((attack * skill correction - defense / 3) * random, 250), without a cap.
        SkillDamageValueType::Rate => ((attack * skill.correction_value
            - defense / DEFENSE_DAMAGE_DIVISOR)
            * random.damage_multiplier())
        .max(MIN_DAMAGE),
    }
}

pub fn validate(skill: &SkillMasterData) -> Result<(), Error> {
    use SkillEffectID as E;
    use SkillMasterDataEffectData as D;
    use SkillTargetRange as R;
    if !skill.activation_rate.is_finite() {
        return Err(Error::InvalidValue("skill activation correction"));
    }
    crate::nonnegative(skill.correction_value, "skill correction")?;
    let attack = skill.effect_id == E::Attack;
    if (attack && skill.target_range == R::Single)
        || (!attack && !matches!(skill.target_range, R::All | R::Single))
    {
        return Err(Error::InvalidValue("skill target range"));
    }
    let valid_data = match (&skill.effect_data, skill.effect_id) {
        (Some(D::StatCorrection(v)), E::Buff | E::Debuff) => v.stat_target != SkillStatTarget::None,
        (Some(D::StatusAbnormality(v)), E::StatusAbnormality) => {
            crate::rate(v.application_rate).is_ok()
        }
        (Some(D::Heal(v)), E::Heal) => crate::rate(v.heal_rate).is_ok(),
        (Some(D::RandomAttack(v)), E::Attack) => skill.target_range == R::Random && v.hit_count > 0,
        (None, E::Attack) => skill.target_range != R::Random,
        _ => false,
    };
    let max = if matches!(skill.effect_id, E::Buff | E::Debuff) {
        BUFF_DEBUFF_ACTIVATION_LIMIT
    } else {
        u32::MAX
    };
    if !valid_data || skill.max_activation_count != max {
        return Err(Error::InvalidValue("skill effect data"));
    }
    if attack
        && skill.damage_value_type == SkillDamageValueType::Fixed
        && !(MIN_DAMAGE..=MAX_NORMAL_DAMAGE).contains(&skill.correction_value)
    {
        return Err(Error::InvalidValue("fixed skill damage"));
    }
    Ok(())
}

pub(crate) fn preferred(c: &CharacterBattle, condition: &SkillTargetConditionData) -> bool {
    match condition.condition_id {
        SkillTargetConditionID::None => false,
        SkillTargetConditionID::Buffed => matches!(
            c.buff_debuff_state,
            BuffDebuffState::Buff | BuffDebuffState::BuffDebuff
        ),
        SkillTargetConditionID::Debuffed => matches!(
            c.buff_debuff_state,
            BuffDebuffState::Debuff | BuffDebuffState::BuffDebuff
        ),
        SkillTargetConditionID::Hp25PercentOrBelow => {
            c.hp.current_hp <= c.hp.max_hp * LOW_HP_TARGET_RATE
        }
        SkillTargetConditionID::StatusAbnormality => {
            status_abnormality::contains(c, condition.status_abnormality_id)
        }
    }
}

/// Candidates must already be filtered by side and living/revival eligibility.
pub(crate) fn targets(
    skill: &SkillMasterData,
    attacker: Position,
    candidates: &[(usize, Position, &CharacterBattle)],
    random: &mut Random,
) -> Vec<usize> {
    use SkillTargetRange as R;
    let mut ordered = candidates.to_vec();
    ordered.sort_by_key(|c| c.1);
    if ordered.is_empty() {
        return Vec::new();
    }
    if skill.target_range == R::Single {
        let preferred: Vec<_> = ordered
            .iter()
            .copied()
            .filter(|c| preferred(c.2, &skill.target_condition))
            .collect();
        if !preferred.is_empty() {
            ordered = preferred;
        }
        shuffle(&mut ordered, random);
        return vec![ordered[0].0];
    }
    let positions: Vec<_> = ordered.iter().map(|c| c.1).collect();
    // Convert the attacker's formation column into the opposing party's coordinate system.
    let preferred_column = if skill.target_side == SkillTargetSide::Enemy {
        crate::formation::LAST_GRID_INDEX - attacker.column()
    } else {
        attacker.column()
    };
    let column = best_column(&positions, preferred_column);
    let row = (0..crate::formation::GRID_SIZE)
        .max_by_key(|&row| {
            (
                positions.iter().filter(|p| p.row() == row).count(),
                std::cmp::Reverse(row),
            )
        })
        .unwrap();
    ordered
        .into_iter()
        .filter(|c| match skill.target_range {
            R::All | R::Random => true,
            R::VerticalColumn => c.1.column() == column,
            R::HorizontalRow => c.1.row() == row,
            R::XShape => {
                c.1.row() == c.1.column()
                    || c.1.row() + c.1.column() == crate::formation::LAST_GRID_INDEX
            }
            R::CrossShape => {
                c.1.row() == crate::formation::CENTER_GRID_INDEX
                    || c.1.column() == crate::formation::CENTER_GRID_INDEX
            }
            R::Single => unreachable!(),
        })
        .map(|c| c.0)
        .collect()
}
