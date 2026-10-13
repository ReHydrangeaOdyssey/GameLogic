use crate::model::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parameter {
    Attack,
    Defense,
    Speed,
    SkillActivationRate,
    MaxTp,
    BattleScore,
    GuildBattleScore,
    CastleBreakScore,
    AssaultCastleBreakRate,
    AssaultCastleBreakScore,
    Hate,
    CastleLevel,
    BpRecovery,
    TpRecovery,
    AttackCountScore,
    CastleBreakScoreLimit,
    HpRecoveryValue,
    ReviveRate,
    AttackTargetRate,
}
impl Parameter {
    pub const ALL: [Self; 19] = [
        Self::Attack,
        Self::Defense,
        Self::Speed,
        Self::SkillActivationRate,
        Self::MaxTp,
        Self::BattleScore,
        Self::GuildBattleScore,
        Self::CastleBreakScore,
        Self::AssaultCastleBreakRate,
        Self::AssaultCastleBreakScore,
        Self::Hate,
        Self::CastleLevel,
        Self::BpRecovery,
        Self::TpRecovery,
        Self::AttackCountScore,
        Self::CastleBreakScoreLimit,
        Self::HpRecoveryValue,
        Self::ReviveRate,
        Self::AttackTargetRate,
    ];
    pub(crate) fn special(self, s: &TacticsBattleSpecialData) -> Option<f32> {
        super::validation::parameters(s.special_type)
            .contains(&self)
            .then(|| self.value(&s.parameters))
    }
    pub(super) fn value(self, p: &TacticsBattleSpecialParameters) -> f32 {
        match self {
            Self::Attack => p.attack,
            Self::Defense => p.defense,
            Self::Speed => p.speed,
            Self::SkillActivationRate => p.skill_activation_rate,
            Self::MaxTp => p.max_tp,
            Self::BattleScore => p.battle_score,
            Self::GuildBattleScore => p.guild_battle_score,
            Self::CastleBreakScore => p.castle_break_score,
            Self::AssaultCastleBreakRate => p.assault_castle_break_rate,
            Self::AssaultCastleBreakScore => p.assault_castle_break_score,
            Self::Hate => p.hate,
            Self::CastleLevel => p.castle_level as f32,
            Self::BpRecovery => p.bp_recovery as f32,
            Self::TpRecovery => p.tp_recovery as f32,
            Self::AttackCountScore => p.attack_count_score,
            Self::CastleBreakScoreLimit => p.castle_break_score_limit,
            Self::HpRecoveryValue => p.hp_recovery_value,
            Self::ReviveRate => p.revive_rate,
            Self::AttackTargetRate => p.attack_target_rate,
        }
    }
}
