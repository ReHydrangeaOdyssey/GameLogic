const MAX_GUILD_PARTY_SIZE: usize = 10;
const RANK_STAT_COUNT: u32 = 4;
const RANK_COUNT: u32 = PartyRank::FMinus as u32 + 1;
const HP_THRESHOLD_COEFFICIENT: f32 = 500.0;
const HP_THRESHOLD_BASE: f32 = 5_000.0;
const COMBAT_STAT_THRESHOLD_COEFFICIENT: f32 = 175.0;
const COMBAT_STAT_THRESHOLD_BASE: f32 = 1_750.0;
const REVIVE_SECONDS_S_OR_HIGHER: u32 = 8;
const REVIVE_SECONDS_S_MINUS_TO_A_PLUS: u32 = 7;
const REVIVE_SECONDS_A_TO_A_MINUS: u32 = 6;
const REVIVE_SECONDS_B_PLUS_OR_LOWER: u32 = 5;

use crate::{Error, model::*};

pub fn calculate(characters: &[PartyCharacterStatus]) -> Result<PartyRank, Error> {
    if characters.is_empty() {
        return Err(Error::EmptyParty);
    }
    if characters.len() > MAX_GUILD_PARTY_SIZE {
        return Err(Error::InvalidValue("party size"));
    }
    let mut hp = 0.0_f32;
    let mut attack = 0.0_f32;
    let mut defense = 0.0_f32;
    let mut speed = 0_u32;
    for c in characters {
        if c.id == u32::MAX {
            return Err(Error::InvalidValue("CharacterID"));
        }
        crate::nonnegative(c.max_hp, "max_hp")?;
        crate::nonnegative(c.attack, "attack")?;
        crate::nonnegative(c.defense, "defense")?;
        hp += c.max_hp;
        attack += c.attack;
        defense += c.defense;
        speed += c.speed as u32;
    }
    // Rank = ceil((HP rank + attack rank + defense rank + floor(mean speed)) / 4).
    let sum = stat_rank(hp.floor(), HP_THRESHOLD_COEFFICIENT, HP_THRESHOLD_BASE)
        + stat_rank(
            attack.floor(),
            COMBAT_STAT_THRESHOLD_COEFFICIENT,
            COMBAT_STAT_THRESHOLD_BASE,
        )
        + stat_rank(
            defense.floor(),
            COMBAT_STAT_THRESHOLD_COEFFICIENT,
            COMBAT_STAT_THRESHOLD_BASE,
        )
        + speed / characters.len() as u32;
    PartyRank::try_from(sum.div_ceil(RANK_STAT_COUNT))
}

fn stat_rank(total: f32, coefficient: f32, base: f32) -> u32 {
    for n in 1..RANK_COUNT {
        // Upper bound = coefficient * n² + base; SS9 has no upper bound.
        if total <= coefficient * (n * n) as f32 + base {
            return RANK_COUNT - n;
        }
    }
    PartyRank::Ss9 as u32
}

pub fn revive_seconds(rank: PartyRank) -> u32 {
    match rank as u32 {
        value if value <= PartyRank::S as u32 => REVIVE_SECONDS_S_OR_HIGHER,
        value if value <= PartyRank::APlus as u32 => REVIVE_SECONDS_S_MINUS_TO_A_PLUS,
        value if value <= PartyRank::AMinus as u32 => REVIVE_SECONDS_A_TO_A_MINUS,
        _ => REVIVE_SECONDS_B_PLUS_OR_LOWER,
    }
}
