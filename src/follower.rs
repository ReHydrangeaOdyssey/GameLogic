pub const FOLLOWER_SLOT_COUNT: usize = 2;
const LOWER_RARITY_SLOT: usize = 1;
const FOLLOWER_STAT_RATE: f32 = 0.1;

use crate::{Error, model::*};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loadout {
    pub character_id: u32,
    pub followers: [Option<u32>; FOLLOWER_SLOT_COUNT],
    pub main_skill_id: u32,
    pub ability_ids: Vec<u32>,
}

/// Constructs shared initial state with follower and equipment validation.
pub fn prepare_character(
    master: &crate::master_data::MasterData,
    loadout: &Loadout,
    owner_player_id: u64,
) -> Result<CharacterBattle, Error> {
    if owner_player_id == u64::MAX {
        return Err(Error::InvalidValue("PlayerID"));
    }
    let body = master.character(loadout.character_id)?;
    let followers = [
        loadout.followers[0]
            .map(|id| master.character(id))
            .transpose()?,
        loadout.followers[1]
            .map(|id| master.character(id))
            .transpose()?,
    ];
    validate_main_skill(body, followers, loadout.main_skill_id)?;
    let abilities = loadout
        .ability_ids
        .iter()
        .map(|&id| master.ability(id))
        .collect::<Result<Vec<_>, _>>()?;
    crate::ability::validate_equipment(body, &abilities)?;
    let status = party_status(body, followers)?;
    Ok(CharacterBattle {
        id: body.id,
        attribute: body.attribute,
        hp: HitPoints {
            max_hp: status.max_hp,
            current_hp: status.max_hp,
        },
        attack: status.attack,
        defense: status.defense,
        speed: status.speed,
        bp: body.bp,
        skill_state: SkillBattleState {
            skill_id: loadout.main_skill_id,
            activation_count: 0,
        },
        tactics_ids: body.tactics_ids.clone(),
        ability_states: loadout
            .ability_ids
            .iter()
            .map(|&ability_id| AbilityBattleState {
                ability_id,
                activation_count: 0,
                activated_this_turn: false,
            })
            .collect(),
        owner_player_id,
        buff_debuff_state: BuffDebuffState::None,
        buff_debuff_effect: BuffDebuffEffectState::default(),
        status_abnormalities: Vec::new(),
    })
}

pub fn party_status(
    body: &CharacterMasterData,
    followers: [Option<&CharacterMasterData>; FOLLOWER_SLOT_COUNT],
) -> Result<PartyCharacterStatus, Error> {
    for (slot, follower) in followers.iter().enumerate() {
        if let Some(follower) = follower
            && (follower.id == body.id
                || follower.rarity > body.rarity
                || (slot == LOWER_RARITY_SLOT && follower.rarity == body.rarity))
        {
            return Err(Error::InvalidFollower);
        }
    }
    if let [Some(a), Some(b)] = followers
        && a.id == b.id
    {
        return Err(Error::InvalidFollower);
    }
    let mut status = PartyCharacterStatus {
        id: body.id,
        max_hp: body.hp as f32,
        attack: body.attack as f32,
        defense: body.defense as f32,
        speed: body.speed,
    };
    // Final stat = body + follower slot 0 * 0.1 + follower slot 1 * 0.1.
    for follower in followers.into_iter().flatten() {
        status.max_hp += follower.hp as f32 * FOLLOWER_STAT_RATE;
        status.attack += follower.attack as f32 * FOLLOWER_STAT_RATE;
        status.defense += follower.defense as f32 * FOLLOWER_STAT_RATE;
    }
    Ok(status)
}

pub fn validate_main_skill(
    body: &CharacterMasterData,
    followers: [Option<&CharacterMasterData>; FOLLOWER_SLOT_COUNT],
    skill_id: u32,
) -> Result<(), Error> {
    party_status(body, followers)?;
    if body.skill_ids.contains(&skill_id)
        || followers
            .into_iter()
            .flatten()
            .any(|f| f.skill_ids.contains(&skill_id))
    {
        Ok(())
    } else {
        Err(Error::InvalidEquipment)
    }
}
