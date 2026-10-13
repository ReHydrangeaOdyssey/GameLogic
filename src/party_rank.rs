use crate::{Error, model::*};

pub fn calculate(characters: &[PartyCharacterStatus]) -> Result<PartyRank, Error> {
    if characters.is_empty() {
        return Err(Error::EmptyParty);
    }
    if characters.len() > 10 {
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
    let sum = stat_rank(hp.floor(), 500.0, 5_000.0)
        + stat_rank(attack.floor(), 175.0, 1_750.0)
        + stat_rank(defense.floor(), 175.0, 1_750.0)
        + speed / characters.len() as u32;
    PartyRank::try_from(sum.div_ceil(4))
}

fn stat_rank(total: f32, coefficient: f32, base: f32) -> u32 {
    for n in 1..=32_u32 {
        // Upper bound = coefficient * n² + base; SS9 has no upper bound.
        if total <= coefficient * (n * n) as f32 + base {
            return 33 - n;
        }
    }
    0
}

pub fn revive_seconds(rank: PartyRank) -> u32 {
    match rank as u32 {
        0..=13 => 8,
        14..=15 => 7,
        16..=17 => 6,
        _ => 5,
    }
}
