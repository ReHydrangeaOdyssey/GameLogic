use super::Parameter;
use crate::model::*;
// Indexed by the specification-defined TacticsBattleSpecialType discriminant.
const RULES: &[(
    TacticsTarget,
    TacticsBattleSpecialTrigger,
    bool,
    &[Parameter],
)] = &[
    // ACCELERATOR
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Speed],
    ),
    // ASSAULT
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack],
    ),
    // ERASE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Interception,
        false,
        &[],
    ),
    // ACE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::Defense],
    ),
    // EXTERLIZE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::AttackCountScore],
    ),
    // EX_DRIVE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::CastleBreak,
        false,
        &[Parameter::CastleBreakScore],
    ),
    // EDGE_NOTE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::CastleBreak,
        false,
        &[Parameter::CastleBreakScore, Parameter::BpRecovery],
    ),
    // ELYSION
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack],
    ),
    // ENDER_BREAK
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[
            Parameter::GuildBattleScore,
            Parameter::CastleBreakScoreLimit,
        ],
    ),
    // ORACLE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::SkillActivationRate],
    ),
    // ORATORIO
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::SkillActivationRate],
    ),
    // CURSE
    (
        TacticsTarget::EnemyGuild,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::SkillActivationRate],
    ),
    // COUNTER
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Interception,
        false,
        &[Parameter::Attack, Parameter::Defense],
    ),
    // CASTLE_WEAKNESS
    (
        TacticsTarget::EnemyGuild,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::CastleLevel],
    ),
    // CASTLE_VEIL
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::CastleLevel],
    ),
    // CLAUSTRUM
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Interception,
        false,
        &[Parameter::Hate, Parameter::Attack, Parameter::Defense],
    ),
    // GRAVITY_ASSAULT
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::Speed],
    ),
    // CLEVER_NOTE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::EnemyAnnihilated,
        false,
        &[Parameter::BattleScore, Parameter::TpRecovery],
    ),
    // JUGGERNAUT
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::EnemyAnnihilated,
        false,
        &[Parameter::BpRecovery],
    ),
    // SHADOW
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::AssaultCastleBreakRate],
    ),
    // STEALTH
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[
            Parameter::AssaultCastleBreakRate,
            Parameter::AssaultCastleBreakScore,
        ],
    ),
    // STREAM
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::Speed],
    ),
    // SLASHER
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::CastleBreak,
        false,
        &[Parameter::CastleBreakScore],
    ),
    // SLOW_RATE
    (
        TacticsTarget::OpponentParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Speed],
    ),
    // TARANTELLA
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::SkillActivationRate, Parameter::Speed],
    ),
    // DIVINE_ACTIVE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::SkillActivationRate, Parameter::MaxTp],
    ),
    // DIVINE_ETOILE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::Defense, Parameter::MaxTp],
    ),
    // DIVINE_THRUST
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::MaxTp],
    ),
    // DIVINE_RAPID
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Speed, Parameter::MaxTp],
    ),
    // BERSERK
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::Defense],
    ),
    // HIDE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::AttackTargetRate],
    ),
    // PANZER
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Defense, Parameter::Speed],
    ),
    // HEAL
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        true,
        &[Parameter::HpRecoveryValue],
    ),
    // PHALANX
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::AssaultCastleBreakRate, Parameter::Defense],
    ),
    // FORCE_OF_WISH
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::None,
        true,
        &[Parameter::BpRecovery],
    ),
    // FORCE_OF_PLAY
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::None,
        true,
        &[Parameter::TpRecovery],
    ),
    // FORTRESS
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Defense],
    ),
    // BLITZ
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[
            Parameter::AssaultCastleBreakRate,
            Parameter::AssaultCastleBreakScore,
        ],
    ),
    // PROVOKE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::AttackTargetRate],
    ),
    // POINT_RISE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::GuildBattleScore],
    ),
    // MENACE
    (
        TacticsTarget::EnemyGuild,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Defense],
    ),
    // RAMPAGE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::BattleScore],
    ),
    // REVIVE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        true,
        &[Parameter::ReviveRate],
    ),
    // RECONTRACT
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::EnemyAnnihilated,
        false,
        &[Parameter::BpRecovery, Parameter::TpRecovery],
    ),
    // RESURRECTION
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::None,
        true,
        &[Parameter::ReviveRate],
    ),
    // RECT_NOTE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::EnemyAnnihilated,
        false,
        &[Parameter::BpRecovery],
    ),
    // WISE_NOTE
    (
        TacticsTarget::SelfParty,
        TacticsBattleSpecialTrigger::EnemyAnnihilated,
        false,
        &[Parameter::BattleScore, Parameter::BpRecovery],
    ),
    // ASSAULT_ORDER
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack],
    ),
    // ACE_ORDER
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::Battle,
        false,
        &[Parameter::Attack, Parameter::Defense],
    ),
    // SHADOW_ORDER
    (
        TacticsTarget::AllyGuild,
        TacticsBattleSpecialTrigger::None,
        false,
        &[Parameter::AssaultCastleBreakRate],
    ),
];
pub(super) fn parameters(kind: TacticsBattleSpecialType) -> &'static [Parameter] {
    RULES[kind as usize].3
}
pub(super) fn valid_special(
    data: &TacticsBattleSpecialData,
    target: TacticsTarget,
    end: TacticsEndType,
) -> bool {
    let rule = &RULES[data.special_type as usize];
    target == rule.0
        && data.trigger == rule.1
        && (end == TacticsEndType::OnActivation) == rule.2
        && Parameter::ALL
            .iter()
            .all(|p| rule.3.contains(p) || p.value(&data.parameters) == 0.0)
}
