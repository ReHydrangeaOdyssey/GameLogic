use crate::{Error, model::*, skill};
#[derive(Clone, Debug, PartialEq)]
pub struct SkillEffectComponent {
    pub code: String,
    pub level: Option<u32>,
    pub effect: SkillMasterData,
    pub random_hit_range: Option<(u32, u32)>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompositeSkill {
    pub main_effect: SkillEffectComponent,
    pub sub_effects: Vec<SkillEffectComponent>,
    pub probability_level: Option<u32>,
}
impl CompositeSkill {
    pub fn validate(&self) -> Result<(), Error> {
        if self.probability_level.is_some_and(|level| level > 4) {
            return Err(Error::InvalidValue("skill probability level"));
        }
        let main = &self.main_effect.effect;
        for component in std::iter::once(&self.main_effect).chain(&self.sub_effects) {
            if component.code.is_empty() || component.effect.id != main.id {
                return Err(Error::InvalidValue("skill component"));
            }
            skill::validate(&component.effect)?;
            match (&component.effect.effect_data, component.random_hit_range) {
                (Some(SkillMasterDataEffectData::RandomAttack(data)), Some((minimum, maximum))) => {
                    if minimum == 0
                        || minimum > maximum
                        || data.hit_count != minimum
                        || maximum
                            .checked_sub(minimum)
                            .and_then(|v| v.checked_add(1))
                            .is_none()
                    {
                        return Err(Error::InvalidValue("skill random hit range"));
                    }
                }
                (_, None) => {}
                _ => return Err(Error::InvalidValue("skill random hit range")),
            }
        }
        Ok(())
    }
}
