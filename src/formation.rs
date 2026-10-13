use crate::{
    Error,
    model::*,
    pseudorandom::{Random, shuffle},
};

pub const GRID_SIZE: u8 = 3;
pub const MAX_PARTY_SIZE: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(u8);

impl Position {
    pub fn new(value: u8) -> Result<Self, Error> {
        if (1..=9).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidFormation)
        }
    }
    pub fn value(self) -> u8 {
        self.0
    }
    /// Zero at the front, two at the rear.
    pub fn row(self) -> u8 {
        (self.0 - 1) / GRID_SIZE
    }
    /// Zero at the left, two at the right in the formation diagram.
    pub fn column(self) -> u8 {
        2 - (self.0 - 1) % GRID_SIZE
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Correction {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub skill_activation_rate: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionEffect {
    pub position: Position,
    pub condition: FormationConditionID,
    pub correction: Correction,
}

pub fn correction(
    position: Position,
    attribute: CharacterAttribute,
    effects: &[PositionEffect],
) -> Correction {
    let mut result = Correction::default();
    for effect in effects
        .iter()
        .filter(|e| e.position == position && matches_condition(e.condition, attribute))
    {
        // Corrections for the same formation position and stat add together.
        result.attack += effect.correction.attack;
        result.defense += effect.correction.defense;
        result.speed += effect.correction.speed;
        result.skill_activation_rate += effect.correction.skill_activation_rate;
    }
    result
}

pub fn matches_condition(condition: FormationConditionID, attribute: CharacterAttribute) -> bool {
    match condition {
        FormationConditionID::None => true,
        FormationConditionID::SlashOnly => attribute == CharacterAttribute::Slash,
        FormationConditionID::PierceOnly => attribute == CharacterAttribute::Pierce,
        FormationConditionID::StrikeOnly => attribute == CharacterAttribute::Strike,
        FormationConditionID::RangedOnly => attribute == CharacterAttribute::Ranged,
    }
}

/// Returns (formation selection ID, position), resolving conflicts in selection-ID order.
pub fn priority_placement(
    preferences: &[(u8, Position)],
    random: &mut Random,
) -> Result<Vec<(u8, Position)>, Error> {
    if preferences.is_empty() || preferences.len() > MAX_PARTY_SIZE {
        return Err(Error::InvalidFormation);
    }
    let mut ordered = preferences.to_vec();
    ordered.sort_by_key(|v| v.0);
    if ordered.iter().any(|p| p.0 == u8::MAX) || ordered.windows(2).any(|p| p[0].0 == p[1].0) {
        return Err(Error::InvalidFormation);
    }
    let mut placed = Vec::new();
    let mut displaced = Vec::new();
    while let Some(&(_, position)) = ordered.first() {
        let mut group: Vec<_> = ordered
            .iter()
            .copied()
            .filter(|p| p.1 == position)
            .collect();
        ordered.retain(|p| p.1 != position);
        shuffle(&mut group, random);
        placed.push(group[0]);
        displaced.extend(group.into_iter().skip(1).map(|p| p.0));
    }
    for id in displaced {
        let position = (1..=9)
            .map(Position)
            .find(|p| !placed.iter().any(|v| v.1 == *p))
            .ok_or(Error::InvalidFormation)?;
        placed.push((id, position));
    }
    placed.sort_by_key(|p| p.0);
    Ok(placed)
}

/// Geometry is evaluated in the target party's formation coordinates.
pub fn normal_targets(
    attribute: CharacterAttribute,
    attacker: Position,
    positions: &[Position],
    origin: Option<Position>,
    single: bool,
) -> Vec<usize> {
    let mut candidates: Vec<_> = (0..positions.len()).collect();
    if single {
        candidates.sort_by_key(|&i| positions[i]);
        candidates.truncate(1);
        return candidates;
    }
    if positions.is_empty() {
        return candidates;
    }
    // Opposing formations face each other: own position 1 shares a lane with enemy position 3.
    let origin_column = origin
        .map(Position::column)
        .unwrap_or(2 - attacker.column());
    match attribute {
        CharacterAttribute::Slash | CharacterAttribute::Ranged => {
            let row = origin.map(Position::row).unwrap_or_else(|| {
                if attribute == CharacterAttribute::Slash {
                    positions.iter().map(|p| p.row()).min().unwrap()
                } else {
                    positions.iter().map(|p| p.row()).max().unwrap()
                }
            });
            candidates.retain(|&i| positions[i].row() == row);
        }
        CharacterAttribute::Pierce => {
            let col = origin
                .map(Position::column)
                .unwrap_or_else(|| best_column(positions, origin_column));
            candidates.retain(|&i| positions[i].column() == col);
        }
        CharacterAttribute::Strike => {
            let row = origin
                .map(Position::row)
                .unwrap_or_else(|| positions.iter().map(|p| p.row()).min().unwrap())
                .min(1);
            let col = (0..2)
                .filter(|&col| origin.is_none_or(|p| col <= p.column() && p.column() < col + 2))
                .max_by_key(|&col| {
                    let hits = positions
                        .iter()
                        .filter(|p| {
                            p.row() >= row
                                && p.row() < row + 2
                                && p.column() >= col
                                && p.column() < col + 2
                        })
                        .count();
                    (
                        hits,
                        col <= origin_column && origin_column < col + 2,
                        std::cmp::Reverse(col),
                    )
                })
                .unwrap();
            candidates.retain(|&i| {
                let p = positions[i];
                p.row() >= row && p.row() < row + 2 && p.column() >= col && p.column() < col + 2
            });
        }
    }
    candidates.sort_by_key(|&i| (positions[i].row(), positions[i].column()));
    candidates
}

pub(crate) fn best_column(positions: &[Position], preferred: u8) -> u8 {
    (0..GRID_SIZE)
        .max_by_key(|&col| {
            (
                positions.iter().filter(|p| p.column() == col).count(),
                col == preferred,
                std::cmp::Reverse(col),
            )
        })
        .unwrap()
}
