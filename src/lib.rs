//! Deterministic game calculations shared by the client and game server.
//! Callers supply validated master data, initial state, time and random state.
#![forbid(unsafe_code)]

pub mod ability;
pub mod enums;
pub mod follower;
pub mod formation;
pub mod master_data;
pub mod model;
pub mod party_rank;
pub mod pseudorandom;
pub mod skill;
pub mod status_abnormality;
pub mod tactics;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidValue(&'static str),
    MissingMasterData { kind: &'static str, id: u32 },
    DuplicateId { kind: &'static str, id: u32 },
    InvalidFormation,
    InvalidFollower,
    InvalidEquipment,
    EmptyParty,
    ArithmeticOverflow,
    TacticsUnavailable,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue(name) => write!(f, "invalid {name}"),
            Self::MissingMasterData { kind, id } => write!(f, "missing {kind} {id}"),
            Self::DuplicateId { kind, id } => write!(f, "duplicate {kind} {id}"),
            other => write!(f, "{other:?}"),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) fn nonnegative(value: f32, name: &'static str) -> Result<(), Error> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(Error::InvalidValue(name))
    }
}

pub(crate) fn rate(value: f32) -> Result<(), Error> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(Error::InvalidValue("rate"))
    }
}
