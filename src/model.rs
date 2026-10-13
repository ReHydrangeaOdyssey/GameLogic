//! Logical data shared by client and server; wire decoding belongs to the caller.
pub use crate::enums::*;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartyCharacterStatus {
    pub id: u32,
    pub max_hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: SpeedRank,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuffDebuffEffectState {
    pub attack_buff: f32,
    pub attack_debuff: f32,
    pub defense_buff: f32,
    pub defense_debuff: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StatusAbnormalityState {
    pub status_id: StatusAbnormalityID,
    pub elapsed_turns: u32,
    pub poison_cycle_turns: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillBattleState {
    pub skill_id: u32,
    pub activation_count: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityBattleState {
    pub ability_id: u32,
    pub activation_count: u32,
    pub activated_this_turn: bool,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharacterBattle {
    pub id: u32,
    pub attribute: CharacterAttribute,
    pub hp: HitPoints,
    pub attack: f32,
    pub defense: f32,
    pub speed: SpeedRank,
    pub bp: u16,
    pub skill_state: SkillBattleState,
    pub tactics_ids: Vec<u32>,
    pub ability_states: Vec<AbilityBattleState>,
    pub owner_player_id: u64,
    pub buff_debuff_state: BuffDebuffState,
    pub buff_debuff_effect: BuffDebuffEffectState,
    pub status_abnormalities: Vec<StatusAbnormalityState>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HitPoints {
    pub max_hp: f32,
    pub current_hp: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsBattleState {
    pub available_uses: u32,
    pub tp_cost: u8,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsBattleSpecialParameters {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub skill_activation_rate: f32,
    pub max_tp: f32,
    pub battle_score: f32,
    pub guild_battle_score: f32,
    pub castle_break_score: f32,
    pub assault_castle_break_rate: f32,
    pub assault_castle_break_score: f32,
    pub hate: f32,
    pub castle_level: i32,
    pub bp_recovery: u32,
    pub tp_recovery: u32,
    pub attack_count_score: f32,
    pub castle_break_score_limit: f32,
    pub hp_recovery_value: f32,
    pub revive_rate: f32,
    pub attack_target_rate: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsBattleSpecialData {
    pub special_type: TacticsBattleSpecialType,
    pub parameters: TacticsBattleSpecialParameters,
    pub trigger: TacticsBattleSpecialTrigger,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsHpRecoveryData {
    pub recovery_type: TacticsHpRecoveryType,
    pub recovery_rate: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsActiveEffectState {
    pub tactics_id: u32,
    pub effect_id: TacticsEffectID,
    pub target: TacticsTarget,
    pub effect_value: f32,
    pub expires_at: u64,
    pub remaining_count: u32,
    pub battle_special: TacticsBattleSpecialData,
    pub count_consume_trigger: TacticsCountConsumeTrigger,
    pub end_type: TacticsEndType,
    pub erase_consumed: bool,
    pub source_player_id: u64,
    pub source_guild_id: u64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProcessedMasterData {
    pub characters: Vec<CharacterMasterData>,
    pub skills: Vec<SkillMasterData>,
    pub abilities: Vec<AbilityMasterData>,
    pub tactics: Vec<TacticsMasterData>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CharacterMasterData {
    pub id: u32,
    pub title: String,
    pub name: String,
    pub rarity: Rarity,
    pub attribute: CharacterAttribute,
    pub hp: u32,
    pub attack: u16,
    pub defense: u16,
    pub speed: SpeedRank,
    pub bp: u16,
    pub skill_ids: Vec<u32>,
    pub ability_ids: Vec<u32>,
    pub tactics_ids: Vec<u32>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillTargetConditionData {
    pub condition_id: SkillTargetConditionID,
    pub status_abnormality_id: StatusAbnormalityID,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillRandomAttackData {
    pub hit_count: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillStatusAbnormalityData {
    pub status_abnormality_id: StatusAbnormalityID,
    pub application_rate: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillStatCorrectionData {
    pub stat_target: SkillStatTarget,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillHealData {
    pub can_heal_incapacitated: bool,
    pub heal_rate: f32,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SkillMasterDataEffectData {
    RandomAttack(SkillRandomAttackData),
    StatusAbnormality(SkillStatusAbnormalityData),
    StatCorrection(SkillStatCorrectionData),
    Heal(SkillHealData),
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillMasterData {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub effect_id: SkillEffectID,
    pub activation_rate: f32,
    pub correction_value: f32,
    pub target_range: SkillTargetRange,
    pub target_condition: SkillTargetConditionData,
    pub max_activation_count: u32,
    pub damage_value_type: SkillDamageValueType,
    pub target_side: SkillTargetSide,
    pub effect_data: Option<SkillMasterDataEffectData>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityActivationConditionData {
    pub condition_id: AbilityConditionID,
    pub condition_value: u32,
    pub turn_timing: AbilityTurnTiming,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityCorrectionData {
    pub correction_value: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityNoParameterData {}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityStatusAbnormalityData {
    pub status: Option<StatusAbnormalityID>,
    pub avoidance_type: Option<AbilityAvoidanceType>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityStatCorrectionData {
    pub attack: f32,
    pub defense: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityIncapacitatedAllyCountStatCorrectionEntry {
    pub incapacitated_ally_count: u32,
    pub attack: f32,
    pub defense: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityIncapacitatedAllyCountStatCorrectionData {
    pub entries: Vec<AbilityIncapacitatedAllyCountStatCorrectionEntry>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum AbilityMasterDataEffectData {
    Correction(AbilityCorrectionData),
    StatusAbnormality(AbilityStatusAbnormalityData),
    StatCorrection(AbilityStatCorrectionData),
    NoParameter(AbilityNoParameterData),
    IncapacitatedAllyCountStatCorrection(AbilityIncapacitatedAllyCountStatCorrectionData),
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AbilityMasterData {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub allowed_attributes: Vec<CharacterAttribute>,
    pub activation_rate: f32,
    pub max_activation_count: u32,
    pub activation_condition: AbilityActivationConditionData,
    pub effect_id: AbilityEffectID,
    pub target: AbilityTarget,
    pub effect_data: Option<AbilityMasterDataEffectData>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TacticsStageEffectDataIncreaseData {
    IncreaseValue(f32),
    IncreaseUintValue(u32),
    BattleSpecialIncrease(TacticsBattleSpecialParameters),
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsStageEffectData {
    pub effect_id: TacticsEffectID,
    pub target: TacticsTarget,
    pub effect_index: u32,
    pub increase_data: Option<TacticsStageEffectDataIncreaseData>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TacticsEffectDataEffectData {
    CorrectionValue(f32),
    UintValue(u32),
    HpRecovery(TacticsHpRecoveryData),
    BattleSpecial(TacticsBattleSpecialData),
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsEffectData {
    pub effect_id: TacticsEffectID,
    pub target: TacticsTarget,
    pub effect_data: Option<TacticsEffectDataEffectData>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TacticsMasterData {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: TacticsCategory,
    pub tp_cost: u8,
    pub stage_effects: Vec<TacticsStageEffectData>,
    pub end_type: TacticsEndType,
    pub duration_seconds: u32,
    pub effect_count: u32,
    pub count_consume_trigger: TacticsCountConsumeTrigger,
    pub effects: Vec<TacticsEffectData>,
    pub use_condition: TacticsUseCondition,
}
