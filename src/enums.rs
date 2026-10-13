//! Logical enums from Specifications/design/system/game_types.proto.
use crate::Error;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SkillEffectID {
    #[default]
    Buff = 0,
    Debuff = 1,
    StatusAbnormality = 2,
    Heal = 3,
    Attack = 4,
}
impl TryFrom<u32> for SkillEffectID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Buff),
            1 => Ok(Self::Debuff),
            2 => Ok(Self::StatusAbnormality),
            3 => Ok(Self::Heal),
            4 => Ok(Self::Attack),
            _ => Err(Error::InvalidValue("SkillEffectID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SkillDamageValueType {
    #[default]
    Rate = 0,
    Fixed = 1,
}
impl TryFrom<u32> for SkillDamageValueType {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Rate),
            1 => Ok(Self::Fixed),
            _ => Err(Error::InvalidValue("SkillDamageValueType")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SkillTargetSide {
    #[default]
    Ally = 0,
    Enemy = 1,
    SelfTarget = 2,
}
impl TryFrom<u32> for SkillTargetSide {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Ally),
            1 => Ok(Self::Enemy),
            2 => Ok(Self::SelfTarget),
            _ => Err(Error::InvalidValue("SkillTargetSide")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum StatusAbnormalityID {
    #[default]
    Poison = 0,
    Blindness = 1,
    Silence = 2,
    RangeAttackDisabled = 3,
    Coma = 4,
}
impl TryFrom<u32> for StatusAbnormalityID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Poison),
            1 => Ok(Self::Blindness),
            2 => Ok(Self::Silence),
            3 => Ok(Self::RangeAttackDisabled),
            4 => Ok(Self::Coma),
            _ => Err(Error::InvalidValue("StatusAbnormalityID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SkillTargetRange {
    #[default]
    All = 0,
    Single = 1,
    Random = 2,
    VerticalColumn = 3,
    HorizontalRow = 4,
    XShape = 5,
    CrossShape = 6,
}
impl TryFrom<u32> for SkillTargetRange {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::All),
            1 => Ok(Self::Single),
            2 => Ok(Self::Random),
            3 => Ok(Self::VerticalColumn),
            4 => Ok(Self::HorizontalRow),
            5 => Ok(Self::XShape),
            6 => Ok(Self::CrossShape),
            _ => Err(Error::InvalidValue("SkillTargetRange")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SkillTargetConditionID {
    #[default]
    None = 0,
    Buffed = 1,
    Debuffed = 2,
    Hp25PercentOrBelow = 3,
    StatusAbnormality = 4,
}
impl TryFrom<u32> for SkillTargetConditionID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Buffed),
            2 => Ok(Self::Debuffed),
            3 => Ok(Self::Hp25PercentOrBelow),
            4 => Ok(Self::StatusAbnormality),
            _ => Err(Error::InvalidValue("SkillTargetConditionID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SkillStatTarget {
    #[default]
    None = 0,
    Attack = 1,
    Defense = 2,
}
impl TryFrom<u32> for SkillStatTarget {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Attack),
            2 => Ok(Self::Defense),
            _ => Err(Error::InvalidValue("SkillStatTarget")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AbilityAvoidanceType {
    #[default]
    NormalAttack = 0,
    StatusAbnormality = 1,
}
impl TryFrom<u32> for AbilityAvoidanceType {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::NormalAttack),
            1 => Ok(Self::StatusAbnormality),
            _ => Err(Error::InvalidValue("AbilityAvoidanceType")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AbilityEffectID {
    #[default]
    Buff = 0,
    Debuff = 1,
    Avoidance = 2,
    Counter = 3,
    AvoidanceDisable = 4,
    CounterDisable = 5,
    StatusAbnormalityAttack = 6,
    DamageIncrease = 7,
    FixedDamageIncrease = 8,
    Heal = 9,
    Cover = 10,
    DrawAggro = 11,
    Pursuit = 12,
    DefenseIgnore = 13,
    TargetHpLowPriority = 14,
    TargetDefenseDownPriority = 15,
    DrawAggroIgnore = 16,
    AvoidanceCounter = 17,
    SurviveAtOneHp = 18,
    IncapacitatedAllyCountStatCorrection = 19,
    CastleBreakDamageIncrease = 20,
}
impl TryFrom<u32> for AbilityEffectID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Buff),
            1 => Ok(Self::Debuff),
            2 => Ok(Self::Avoidance),
            3 => Ok(Self::Counter),
            4 => Ok(Self::AvoidanceDisable),
            5 => Ok(Self::CounterDisable),
            6 => Ok(Self::StatusAbnormalityAttack),
            7 => Ok(Self::DamageIncrease),
            8 => Ok(Self::FixedDamageIncrease),
            9 => Ok(Self::Heal),
            10 => Ok(Self::Cover),
            11 => Ok(Self::DrawAggro),
            12 => Ok(Self::Pursuit),
            13 => Ok(Self::DefenseIgnore),
            14 => Ok(Self::TargetHpLowPriority),
            15 => Ok(Self::TargetDefenseDownPriority),
            16 => Ok(Self::DrawAggroIgnore),
            17 => Ok(Self::AvoidanceCounter),
            18 => Ok(Self::SurviveAtOneHp),
            19 => Ok(Self::IncapacitatedAllyCountStatCorrection),
            20 => Ok(Self::CastleBreakDamageIncrease),
            _ => Err(Error::InvalidValue("AbilityEffectID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsEffectID {
    #[default]
    AttackCorrection = 0,
    DefenseCorrection = 1,
    SpeedCorrection = 2,
    ScoreCorrection = 3,
    CastleDefenseCorrection = 4,
    ScoreLimitCorrection = 5,
    MaxTpCorrection = 6,
    BpRecovery = 7,
    HpRecovery = 8,
    AssaultCastleBreakRateCorrection = 9,
    BattleSpecial = 10,
    OpponentSortieSelectionRateCorrection = 11,
}
impl TryFrom<u32> for TacticsEffectID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::AttackCorrection),
            1 => Ok(Self::DefenseCorrection),
            2 => Ok(Self::SpeedCorrection),
            3 => Ok(Self::ScoreCorrection),
            4 => Ok(Self::CastleDefenseCorrection),
            5 => Ok(Self::ScoreLimitCorrection),
            6 => Ok(Self::MaxTpCorrection),
            7 => Ok(Self::BpRecovery),
            8 => Ok(Self::HpRecovery),
            9 => Ok(Self::AssaultCastleBreakRateCorrection),
            10 => Ok(Self::BattleSpecial),
            11 => Ok(Self::OpponentSortieSelectionRateCorrection),
            _ => Err(Error::InvalidValue("TacticsEffectID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsHpRecoveryType {
    #[default]
    IncapacitatedFull = 0,
    PositiveHpRate = 1,
}
impl TryFrom<u32> for TacticsHpRecoveryType {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::IncapacitatedFull),
            1 => Ok(Self::PositiveHpRate),
            _ => Err(Error::InvalidValue("TacticsHpRecoveryType")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsBattleSpecialType {
    #[default]
    Accelerator = 0,
    Assault = 1,
    Erase = 2,
    Ace = 3,
    Exterlize = 4,
    ExDrive = 5,
    EdgeNote = 6,
    Elysion = 7,
    EnderBreak = 8,
    Oracle = 9,
    Oratorio = 10,
    Curse = 11,
    Counter = 12,
    CastleWeakness = 13,
    CastleVeil = 14,
    Claustrum = 15,
    GravityAssault = 16,
    CleverNote = 17,
    Juggernaut = 18,
    Shadow = 19,
    Stealth = 20,
    Stream = 21,
    Slasher = 22,
    SlowRate = 23,
    Tarantella = 24,
    DivineActive = 25,
    DivineEtoile = 26,
    DivineThrust = 27,
    DivineRapid = 28,
    Berserk = 29,
    Hide = 30,
    Panzer = 31,
    Heal = 32,
    Phalanx = 33,
    ForceOfWish = 34,
    ForceOfPlay = 35,
    Fortress = 36,
    Blitz = 37,
    Provoke = 38,
    PointRise = 39,
    Menace = 40,
    Rampage = 41,
    Revive = 42,
    Recontract = 43,
    Resurrection = 44,
    RectNote = 45,
    WiseNote = 46,
    AssaultOrder = 47,
    AceOrder = 48,
    ShadowOrder = 49,
}
impl TryFrom<u32> for TacticsBattleSpecialType {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Accelerator),
            1 => Ok(Self::Assault),
            2 => Ok(Self::Erase),
            3 => Ok(Self::Ace),
            4 => Ok(Self::Exterlize),
            5 => Ok(Self::ExDrive),
            6 => Ok(Self::EdgeNote),
            7 => Ok(Self::Elysion),
            8 => Ok(Self::EnderBreak),
            9 => Ok(Self::Oracle),
            10 => Ok(Self::Oratorio),
            11 => Ok(Self::Curse),
            12 => Ok(Self::Counter),
            13 => Ok(Self::CastleWeakness),
            14 => Ok(Self::CastleVeil),
            15 => Ok(Self::Claustrum),
            16 => Ok(Self::GravityAssault),
            17 => Ok(Self::CleverNote),
            18 => Ok(Self::Juggernaut),
            19 => Ok(Self::Shadow),
            20 => Ok(Self::Stealth),
            21 => Ok(Self::Stream),
            22 => Ok(Self::Slasher),
            23 => Ok(Self::SlowRate),
            24 => Ok(Self::Tarantella),
            25 => Ok(Self::DivineActive),
            26 => Ok(Self::DivineEtoile),
            27 => Ok(Self::DivineThrust),
            28 => Ok(Self::DivineRapid),
            29 => Ok(Self::Berserk),
            30 => Ok(Self::Hide),
            31 => Ok(Self::Panzer),
            32 => Ok(Self::Heal),
            33 => Ok(Self::Phalanx),
            34 => Ok(Self::ForceOfWish),
            35 => Ok(Self::ForceOfPlay),
            36 => Ok(Self::Fortress),
            37 => Ok(Self::Blitz),
            38 => Ok(Self::Provoke),
            39 => Ok(Self::PointRise),
            40 => Ok(Self::Menace),
            41 => Ok(Self::Rampage),
            42 => Ok(Self::Revive),
            43 => Ok(Self::Recontract),
            44 => Ok(Self::Resurrection),
            45 => Ok(Self::RectNote),
            46 => Ok(Self::WiseNote),
            47 => Ok(Self::AssaultOrder),
            48 => Ok(Self::AceOrder),
            49 => Ok(Self::ShadowOrder),
            _ => Err(Error::InvalidValue("TacticsBattleSpecialType")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsBattleSpecialTrigger {
    #[default]
    None = 0,
    Battle = 1,
    EnemyAnnihilated = 2,
    CastleBreak = 3,
    AssaultCastleBreak = 4,
    Interception = 5,
}
impl TryFrom<u32> for TacticsBattleSpecialTrigger {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Battle),
            2 => Ok(Self::EnemyAnnihilated),
            3 => Ok(Self::CastleBreak),
            4 => Ok(Self::AssaultCastleBreak),
            5 => Ok(Self::Interception),
            _ => Err(Error::InvalidValue("TacticsBattleSpecialTrigger")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FormationConditionID {
    #[default]
    None = 0,
    SlashOnly = 1,
    PierceOnly = 2,
    StrikeOnly = 3,
    RangedOnly = 4,
}
impl TryFrom<u32> for FormationConditionID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::SlashOnly),
            2 => Ok(Self::PierceOnly),
            3 => Ok(Self::StrikeOnly),
            4 => Ok(Self::RangedOnly),
            _ => Err(Error::InvalidValue("FormationConditionID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AbilityTarget {
    #[default]
    None = 0,
    SelfTarget = 1,
    AllySingle = 2,
    AllyAll = 3,
    EnemySingle = 4,
    EnemyAll = 5,
}
impl TryFrom<u32> for AbilityTarget {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::SelfTarget),
            2 => Ok(Self::AllySingle),
            3 => Ok(Self::AllyAll),
            4 => Ok(Self::EnemySingle),
            5 => Ok(Self::EnemyAll),
            _ => Err(Error::InvalidValue("AbilityTarget")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AbilityConditionID {
    #[default]
    BattleStart = 0,
    Incapacitated = 1,
    NormalAttack = 2,
    EveryNTurns = 3,
    Attacked = 4,
    HpAtOrBelowThreshold = 5,
    CastleBreak = 6,
    HpFullNormalAttack = 7,
    SingleTargetNormalAttack = 8,
    SkillAttack = 9,
    AllyAttack = 10,
    AllyHpAtOrBelowThresholdAttacked = 11,
    EnemyNormalAttackTargeting = 12,
    LethalDamage = 13,
    StatusAbnormalityApplication = 14,
    TargetHpAtOrBelowThresholdNormalAttack = 15,
}
impl TryFrom<u32> for AbilityConditionID {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::BattleStart),
            1 => Ok(Self::Incapacitated),
            2 => Ok(Self::NormalAttack),
            3 => Ok(Self::EveryNTurns),
            4 => Ok(Self::Attacked),
            5 => Ok(Self::HpAtOrBelowThreshold),
            6 => Ok(Self::CastleBreak),
            7 => Ok(Self::HpFullNormalAttack),
            8 => Ok(Self::SingleTargetNormalAttack),
            9 => Ok(Self::SkillAttack),
            10 => Ok(Self::AllyAttack),
            11 => Ok(Self::AllyHpAtOrBelowThresholdAttacked),
            12 => Ok(Self::EnemyNormalAttackTargeting),
            13 => Ok(Self::LethalDamage),
            14 => Ok(Self::StatusAbnormalityApplication),
            15 => Ok(Self::TargetHpAtOrBelowThresholdNormalAttack),
            _ => Err(Error::InvalidValue("AbilityConditionID")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum AbilityTurnTiming {
    #[default]
    TurnStart = 0,
    BeforeAction = 1,
    AfterAction = 2,
    TurnEnd = 3,
}
impl TryFrom<u32> for AbilityTurnTiming {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::TurnStart),
            1 => Ok(Self::BeforeAction),
            2 => Ok(Self::AfterAction),
            3 => Ok(Self::TurnEnd),
            _ => Err(Error::InvalidValue("AbilityTurnTiming")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsTarget {
    #[default]
    SelfParty = 0,
    OpponentParty = 1,
    AllyGuild = 2,
    EnemyGuild = 3,
    CastleBreak = 4,
}
impl TryFrom<u32> for TacticsTarget {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::SelfParty),
            1 => Ok(Self::OpponentParty),
            2 => Ok(Self::AllyGuild),
            3 => Ok(Self::EnemyGuild),
            4 => Ok(Self::CastleBreak),
            _ => Err(Error::InvalidValue("TacticsTarget")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsUseCondition {
    #[default]
    None = 0,
    AllAnnihilated = 1,
}
impl TryFrom<u32> for TacticsUseCondition {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::AllAnnihilated),
            _ => Err(Error::InvalidValue("TacticsUseCondition")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsEndType {
    #[default]
    Duration = 0,
    Count = 1,
    OnActivation = 2,
}
impl TryFrom<u32> for TacticsEndType {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Duration),
            1 => Ok(Self::Count),
            2 => Ok(Self::OnActivation),
            _ => Err(Error::InvalidValue("TacticsEndType")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsCountConsumeTrigger {
    #[default]
    CastleBreak = 0,
    Annihilation = 1,
    AnnihilationAllEnemies = 2,
    Sortie = 3,
}
impl TryFrom<u32> for TacticsCountConsumeTrigger {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::CastleBreak),
            1 => Ok(Self::Annihilation),
            2 => Ok(Self::AnnihilationAllEnemies),
            3 => Ok(Self::Sortie),
            _ => Err(Error::InvalidValue("TacticsCountConsumeTrigger")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Rarity {
    #[default]
    N = 0,
    R = 1,
    Sr = 2,
    Ssr = 3,
    Ur = 4,
}
impl TryFrom<u32> for Rarity {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::N),
            1 => Ok(Self::R),
            2 => Ok(Self::Sr),
            3 => Ok(Self::Ssr),
            4 => Ok(Self::Ur),
            _ => Err(Error::InvalidValue("Rarity")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CharacterAttribute {
    #[default]
    Slash = 0,
    Pierce = 1,
    Strike = 2,
    Ranged = 3,
}
impl TryFrom<u32> for CharacterAttribute {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Slash),
            1 => Ok(Self::Pierce),
            2 => Ok(Self::Strike),
            3 => Ok(Self::Ranged),
            _ => Err(Error::InvalidValue("CharacterAttribute")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SpeedRank {
    #[default]
    Ss9 = 0,
    Ss8 = 1,
    Ss7 = 2,
    Ss6 = 3,
    Ss5 = 4,
    Ss4 = 5,
    Ss3 = 6,
    Ss2 = 7,
    Ss1 = 8,
    SsPlus = 9,
    Ss = 10,
    SsMinus = 11,
    SPlus = 12,
    S = 13,
    SMinus = 14,
    APlus = 15,
    A = 16,
    AMinus = 17,
    BPlus = 18,
    B = 19,
    BMinus = 20,
    CPlus = 21,
    C = 22,
    CMinus = 23,
    DPlus = 24,
    D = 25,
    DMinus = 26,
    EPlus = 27,
    E = 28,
    EMinus = 29,
    FPlus = 30,
    F = 31,
    FMinus = 32,
}
impl TryFrom<u32> for SpeedRank {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Ss9),
            1 => Ok(Self::Ss8),
            2 => Ok(Self::Ss7),
            3 => Ok(Self::Ss6),
            4 => Ok(Self::Ss5),
            5 => Ok(Self::Ss4),
            6 => Ok(Self::Ss3),
            7 => Ok(Self::Ss2),
            8 => Ok(Self::Ss1),
            9 => Ok(Self::SsPlus),
            10 => Ok(Self::Ss),
            11 => Ok(Self::SsMinus),
            12 => Ok(Self::SPlus),
            13 => Ok(Self::S),
            14 => Ok(Self::SMinus),
            15 => Ok(Self::APlus),
            16 => Ok(Self::A),
            17 => Ok(Self::AMinus),
            18 => Ok(Self::BPlus),
            19 => Ok(Self::B),
            20 => Ok(Self::BMinus),
            21 => Ok(Self::CPlus),
            22 => Ok(Self::C),
            23 => Ok(Self::CMinus),
            24 => Ok(Self::DPlus),
            25 => Ok(Self::D),
            26 => Ok(Self::DMinus),
            27 => Ok(Self::EPlus),
            28 => Ok(Self::E),
            29 => Ok(Self::EMinus),
            30 => Ok(Self::FPlus),
            31 => Ok(Self::F),
            32 => Ok(Self::FMinus),
            _ => Err(Error::InvalidValue("SpeedRank")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BuffDebuffState {
    #[default]
    None = 0,
    Buff = 1,
    Debuff = 2,
    BuffDebuff = 3,
}
impl TryFrom<u32> for BuffDebuffState {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Buff),
            2 => Ok(Self::Debuff),
            3 => Ok(Self::BuffDebuff),
            _ => Err(Error::InvalidValue("BuffDebuffState")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PartyRank {
    #[default]
    Ss9 = 0,
    Ss8 = 1,
    Ss7 = 2,
    Ss6 = 3,
    Ss5 = 4,
    Ss4 = 5,
    Ss3 = 6,
    Ss2 = 7,
    Ss1 = 8,
    SsPlus = 9,
    Ss = 10,
    SsMinus = 11,
    SPlus = 12,
    S = 13,
    SMinus = 14,
    APlus = 15,
    A = 16,
    AMinus = 17,
    BPlus = 18,
    B = 19,
    BMinus = 20,
    CPlus = 21,
    C = 22,
    CMinus = 23,
    DPlus = 24,
    D = 25,
    DMinus = 26,
    EPlus = 27,
    E = 28,
    EMinus = 29,
    FPlus = 30,
    F = 31,
    FMinus = 32,
}
impl TryFrom<u32> for PartyRank {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Ss9),
            1 => Ok(Self::Ss8),
            2 => Ok(Self::Ss7),
            3 => Ok(Self::Ss6),
            4 => Ok(Self::Ss5),
            5 => Ok(Self::Ss4),
            6 => Ok(Self::Ss3),
            7 => Ok(Self::Ss2),
            8 => Ok(Self::Ss1),
            9 => Ok(Self::SsPlus),
            10 => Ok(Self::Ss),
            11 => Ok(Self::SsMinus),
            12 => Ok(Self::SPlus),
            13 => Ok(Self::S),
            14 => Ok(Self::SMinus),
            15 => Ok(Self::APlus),
            16 => Ok(Self::A),
            17 => Ok(Self::AMinus),
            18 => Ok(Self::BPlus),
            19 => Ok(Self::B),
            20 => Ok(Self::BMinus),
            21 => Ok(Self::CPlus),
            22 => Ok(Self::C),
            23 => Ok(Self::CMinus),
            24 => Ok(Self::DPlus),
            25 => Ok(Self::D),
            26 => Ok(Self::DMinus),
            27 => Ok(Self::EPlus),
            28 => Ok(Self::E),
            29 => Ok(Self::EMinus),
            30 => Ok(Self::FPlus),
            31 => Ok(Self::F),
            32 => Ok(Self::FMinus),
            _ => Err(Error::InvalidValue("PartyRank")),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TacticsCategory {
    #[default]
    Buff = 0,
    Debuff = 1,
    ScoreUp = 2,
    Heal = 3,
    Special = 4,
}
impl TryFrom<u32> for TacticsCategory {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Buff),
            1 => Ok(Self::Debuff),
            2 => Ok(Self::ScoreUp),
            3 => Ok(Self::Heal),
            4 => Ok(Self::Special),
            _ => Err(Error::InvalidValue("TacticsCategory")),
        }
    }
}
