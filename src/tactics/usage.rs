use super::{Source, activate, dedicated_seed, recover_hp, stage_effects};
use crate::{Error, model::*, pseudorandom::Random};

#[derive(Clone, Copy, Debug)]
pub struct UseContext {
    pub source: Source,
    pub now: u64,
    pub owned_count: u32,
    pub all_annihilated: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseResult {
    pub seed: u64,
    pub immediate_effects: Vec<TacticsEffectData>,
}

/// Pure state transition. Failure leaves TP, uses, effects and the main PRNG unchanged.
pub fn use_tactics(
    master: &TacticsMasterData,
    context: UseContext,
    state: &mut TacticsBattleState,
    tp: &mut u8,
    active: &mut Vec<TacticsActiveEffectState>,
    random: &mut Random,
) -> Result<UseResult, Error> {
    if state.available_uses == 0
        || *tp < state.tp_cost
        || (master.use_condition == TacticsUseCondition::AllAnnihilated && !context.all_annihilated)
    {
        return Err(Error::TacticsUnavailable);
    }
    let effects = stage_effects(master, context.owned_count)?;
    activate(master, &effects, active, context.source, context.now)?;
    let seed = dedicated_seed(&effects, random);
    *tp -= state.tp_cost;
    state.available_uses -= 1;
    Ok(UseResult {
        seed,
        immediate_effects: if master.end_type == TacticsEndType::OnActivation {
            effects
        } else {
            Vec::new()
        },
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartyResources {
    pub player_id: u64,
    pub guild_id: u64,
    pub bp: u16,
    pub max_bp: u16,
    pub tp: u8,
    pub max_tp: u8,
    pub characters: Vec<(u8, HitPoints)>,
}

/// Apply the returned immediate effects using one dedicated PRNG, never the main PRNG.
/// The caller binds OPPONENT_PARTY explicitly for the current encounter.
pub fn apply_immediate(
    result: &UseResult,
    source: Source,
    opponent_id: Option<u64>,
    parties: &mut [PartyResources],
) -> Result<(), Error> {
    let mut next = parties.to_vec();
    validate_parties(&next)?;
    let mut random = Random::new(result.seed);
    let mut order: Vec<_> = (0..next.len()).collect();
    order.sort_by_key(|&i| next[i].player_id);
    for effect in &result.immediate_effects {
        for &i in &order {
            if !target_matches(effect.target, &next[i], source, opponent_id) {
                continue;
            }
            apply_effect(effect, source, &mut next[i], &mut random)?;
        }
    }
    parties.clone_from_slice(&next);
    Ok(())
}

fn validate_parties(parties: &[PartyResources]) -> Result<(), Error> {
    let mut players = std::collections::BTreeSet::new();
    for party in parties {
        if !players.insert(party.player_id) || party.bp > party.max_bp || party.tp > party.max_tp {
            return Err(Error::InvalidValue("party resources"));
        }
        let mut slots = std::collections::BTreeSet::new();
        for (id, hp) in &party.characters {
            crate::nonnegative(hp.current_hp, "current_hp")?;
            crate::nonnegative(hp.max_hp, "max_hp")?;
            if *id == u8::MAX || !slots.insert(*id) || hp.current_hp > hp.max_hp {
                return Err(Error::InvalidValue("party HP"));
            }
        }
    }
    Ok(())
}

fn target_matches(
    target: TacticsTarget,
    party: &PartyResources,
    source: Source,
    opponent: Option<u64>,
) -> bool {
    match target {
        TacticsTarget::SelfParty => party.player_id == source.player_id,
        TacticsTarget::OpponentParty => Some(party.player_id) == opponent,
        TacticsTarget::AllyGuild => party.guild_id == source.guild_id,
        TacticsTarget::EnemyGuild => party.guild_id != source.guild_id,
        TacticsTarget::CastleBreak => false,
    }
}

fn apply_effect(
    effect: &TacticsEffectData,
    source: Source,
    party: &mut PartyResources,
    random: &mut Random,
) -> Result<(), Error> {
    use TacticsEffectDataEffectData as D;
    match &effect.effect_data {
        Some(D::UintValue(value)) if effect.effect_id == TacticsEffectID::BpRecovery => {
            // Recover fixed BP, capped at the player's maximum.
            party.bp =
                (u32::from(party.bp).saturating_add(*value)).min(u32::from(party.max_bp)) as u16;
        }
        Some(D::HpRecovery(value)) => {
            for (_, hp) in &mut party.characters {
                recover_hp(hp, value);
            }
        }
        Some(D::BattleSpecial(value)) => special(value, source, party, random)?,
        _ => return Err(Error::InvalidValue("immediate tactics effect")),
    }
    Ok(())
}

fn special(
    value: &TacticsBattleSpecialData,
    source: Source,
    party: &mut PartyResources,
    random: &mut Random,
) -> Result<(), Error> {
    use TacticsBattleSpecialType as T;
    let p = &value.parameters;
    match value.special_type {
        T::ForceOfWish => {
            party.bp = u32::from(party.bp)
                .saturating_add(p.bp_recovery)
                .min(u32::from(party.max_bp)) as u16
        }
        T::ForceOfPlay => {
            if source.player_id != party.player_id {
                party.tp = u32::from(party.tp)
                    .saturating_add(p.tp_recovery)
                    .min(u32::from(party.max_tp)) as u8;
            }
        }
        T::Heal => {
            for (_, hp) in &mut party.characters {
                if hp.current_hp > 0.0 {
                    hp.heal(p.hp_recovery_value);
                }
            }
        }
        T::Revive | T::Resurrection => {
            crate::rate(p.revive_rate)?;
            let mut indices: Vec<_> = (0..party.characters.len()).collect();
            indices.sort_by_key(|&i| party.characters[i].0);
            for i in indices {
                let hp = &mut party.characters[i].1;
                if hp.current_hp == 0.0 && random.probability(p.revive_rate) {
                    hp.current_hp = hp.max_hp;
                }
            }
        }
        _ => return Err(Error::InvalidValue("immediate battle special")),
    }
    Ok(())
}
