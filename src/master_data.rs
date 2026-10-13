pub(crate) const MAX_CHARACTER_BP: u16 = 99;
const MAX_CHARACTER_TACTICS: usize = 2;

use crate::{Error, model::*};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct MasterData {
    characters: BTreeMap<u32, CharacterMasterData>,
    skills: BTreeMap<u32, SkillMasterData>,
    abilities: BTreeMap<u32, AbilityMasterData>,
    tactics: BTreeMap<u32, TacticsMasterData>,
}

fn insert<T>(
    map: &mut BTreeMap<u32, T>,
    id: u32,
    value: T,
    kind: &'static str,
    reserved: bool,
) -> Result<(), Error> {
    if reserved && id == u32::MAX {
        return Err(Error::InvalidValue("reserved master ID"));
    }
    if map.insert(id, value).is_some() {
        return Err(Error::DuplicateId { kind, id });
    }
    Ok(())
}

impl MasterData {
    pub fn new(data: ProcessedMasterData) -> Result<Self, Error> {
        let mut result = Self {
            characters: BTreeMap::new(),
            skills: BTreeMap::new(),
            abilities: BTreeMap::new(),
            tactics: BTreeMap::new(),
        };
        for skill in data.skills {
            crate::skill::validate(&skill)?;
            insert(&mut result.skills, skill.id, skill, "skill", true)?;
        }
        for ability in data.abilities {
            crate::ability::validate(&ability)?;
            insert(&mut result.abilities, ability.id, ability, "ability", true)?;
        }
        for tactics in data.tactics {
            crate::tactics::validate(&tactics)?;
            insert(&mut result.tactics, tactics.id, tactics, "tactics", false)?;
        }
        for character in data.characters {
            if character.skill_ids.is_empty()
                || character.tactics_ids.len() > MAX_CHARACTER_TACTICS
                || character.bp > MAX_CHARACTER_BP
            {
                return Err(Error::InvalidValue("character master"));
            }
            for &id in &character.skill_ids {
                result.skill(id)?;
            }
            for &id in &character.ability_ids {
                result.ability(id)?;
            }
            for &id in &character.tactics_ids {
                result.tactics(id)?;
            }
            insert(
                &mut result.characters,
                character.id,
                character,
                "character",
                true,
            )?;
        }
        Ok(result)
    }

    pub fn character(&self, id: u32) -> Result<&CharacterMasterData, Error> {
        self.characters.get(&id).ok_or(Error::MissingMasterData {
            kind: "character",
            id,
        })
    }
    pub fn skill(&self, id: u32) -> Result<&SkillMasterData, Error> {
        self.skills
            .get(&id)
            .ok_or(Error::MissingMasterData { kind: "skill", id })
    }
    pub fn ability(&self, id: u32) -> Result<&AbilityMasterData, Error> {
        self.abilities.get(&id).ok_or(Error::MissingMasterData {
            kind: "ability",
            id,
        })
    }
    pub fn tactics(&self, id: u32) -> Result<&TacticsMasterData, Error> {
        self.tactics.get(&id).ok_or(Error::MissingMasterData {
            kind: "tactics",
            id,
        })
    }
}
