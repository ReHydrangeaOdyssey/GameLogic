use super::{Battle, Event, tactical::Stat};
use crate::{ability, formation, model::*, pseudorandom::shuffle, skill, status_abnormality};

#[derive(Clone, Copy)]
struct AttackContext {
    actor: usize,
    cover: Option<usize>,
    ignore_defense: bool,
    damage_multiplier: f32,
}

impl Battle<'_> {
    pub(super) fn action(&mut self, actor: usize) {
        let skill = self.skill(actor).clone();
        let ch = &self.combatants[actor].character;
        let rate = skill::activation_rate(
            &skill,
            self.combatants[actor].formation.skill_activation_rate,
            self.tactics_correction(actor, Stat::SkillRate),
        );
        if ch.skill_state.activation_count < skill.max_activation_count
            && !status_abnormality::contains(ch, StatusAbnormalityID::Silence)
            && self.random.probability(rate)
        {
            self.combatants[actor]
                .character
                .skill_state
                .activation_count = ch.skill_state.activation_count.saturating_add(1);
            self.events.push(Event::Skill {
                actor,
                skill_id: skill.id,
            });
            self.skill_action(actor, &skill);
        } else {
            self.normal_action(actor);
        }
    }

    fn skill_action(&mut self, actor: usize, skill: &SkillMasterData) {
        let side = self.combatants[actor].side;
        let revival = matches!(&skill.effect_data, Some(SkillMasterDataEffectData::Heal(v)) if v.can_heal_incapacitated);
        let candidates: Vec<_> = self
            .combatants
            .iter()
            .enumerate()
            .filter(|(i, c)| {
                let side_match = match skill.target_side {
                    SkillTargetSide::Ally => c.side == side,
                    SkillTargetSide::Enemy => c.side != side,
                    SkillTargetSide::SelfTarget => *i == actor,
                };
                side_match && (c.character.hp.current_hp == 0.0) == revival
            })
            .map(|(i, c)| (i, c.position, &c.character))
            .collect();
        let targets = skill::targets(
            skill,
            self.combatants[actor].position,
            &candidates,
            &mut self.random,
        );
        if targets.is_empty() {
            return;
        }
        let fixed = if skill.effect_id == SkillEffectID::Attack
            && skill.damage_value_type == SkillDamageValueType::Fixed
        {
            self.activate_effect(actor, AbilityEffectID::FixedDamageIncrease)
                .map(|s| ability::correction(self.ability(actor, s)))
                .unwrap_or(0.0)
        } else {
            0.0
        };
        if let Some(SkillMasterDataEffectData::RandomAttack(data)) = &skill.effect_data {
            for _ in 0..data.hit_count {
                let target = targets[self.random.next_bounded(targets.len() as u32) as usize];
                self.skill_hit(actor, target, skill, fixed);
            }
        } else {
            for target in targets {
                self.skill_hit(actor, target, skill, fixed);
            }
        }
    }

    fn skill_hit(&mut self, actor: usize, target: usize, skill: &SkillMasterData, fixed: f32) {
        use SkillMasterDataEffectData as D;
        if skill.effect_id == SkillEffectID::Attack {
            let damage = skill::attack_damage(
                skill,
                self.stat(actor, true),
                self.stat(target, false),
                fixed,
                &mut self.random,
            );
            self.apply_damage(actor, target, damage);
            self.incapacitated(target);
            return;
        }
        match &skill.effect_data {
            Some(D::StatCorrection(v)) => {
                let ch = &mut self.combatants[target].character;
                let b = &mut ch.buff_debuff_effect;
                match (skill.effect_id, v.stat_target) {
                    (SkillEffectID::Buff, SkillStatTarget::Attack) => {
                        b.attack_buff += skill.correction_value
                    }
                    (SkillEffectID::Buff, SkillStatTarget::Defense) => {
                        b.defense_buff += skill.correction_value
                    }
                    (SkillEffectID::Debuff, SkillStatTarget::Attack) => {
                        b.attack_debuff += skill.correction_value
                    }
                    (SkillEffectID::Debuff, SkillStatTarget::Defense) => {
                        b.defense_debuff += skill.correction_value
                    }
                    _ => unreachable!("validated skill data"),
                }
                status_abnormality::update_buff_state(ch);
            }
            Some(D::StatusAbnormality(v)) if self.random.probability(v.application_rate) => {
                self.add_status(actor, target, v.status_abnormality_id)
            }
            Some(D::Heal(v)) => {
                let hp = &mut self.combatants[target].character.hp;
                // Skill recovery = target maximum HP * heal rate.
                let amount = hp.max_hp * v.heal_rate;
                hp.heal(amount);
                self.combatants[target]
                    .character
                    .status_abnormalities
                    .retain(|s| s.status_id != StatusAbnormalityID::Poison);
                if self.alive(target) {
                    self.death_processed[target] = false;
                }
                self.events.push(Event::Heal {
                    actor,
                    target,
                    amount,
                });
            }
            _ => {}
        }
    }

    fn normal_action(&mut self, actor: usize) {
        let ignore_aggro = self
            .activate_effect(actor, AbilityEffectID::DrawAggroIgnore)
            .is_some();
        let aggro = if ignore_aggro {
            None
        } else {
            self.choose_effect(actor, AbilityEffectID::DrawAggro, false, |_, _, _| true)
                .map(|v| v.0)
        };
        let origin = aggro
            .or_else(|| self.priority_target(actor))
            .map(|i| self.combatants[i].position);
        let enemies: Vec<_> = (0..self.combatants.len())
            .filter(|&i| self.combatants[i].side != self.combatants[actor].side && self.alive(i))
            .collect();
        let positions: Vec<_> = enemies
            .iter()
            .map(|&i| self.combatants[i].position)
            .collect();
        let single = status_abnormality::contains(
            &self.combatants[actor].character,
            StatusAbnormalityID::RangeAttackDisabled,
        );
        let targets: Vec<_> = formation::normal_targets(
            self.combatants[actor].character.attribute,
            self.combatants[actor].position,
            &positions,
            origin,
            single,
        )
        .into_iter()
        .map(|i| enemies[i])
        .collect();
        let cover = self
            .choose_effect(actor, AbilityEffectID::Cover, false, |battle, i, s| {
                targets.iter().any(|&t| {
                    t != i
                        && battle.hp_threshold(
                            t,
                            battle.ability(i, s).activation_condition.condition_value,
                        )
                })
            })
            .map(|v| v.0);
        let mut context = AttackContext {
            actor,
            cover,
            ignore_defense: false,
            damage_multiplier: 1.0,
        };
        for slot in 0..self.combatants[actor].character.ability_states.len() {
            let a = self.ability(actor, slot);
            let valid = match a.activation_condition.condition_id {
                AbilityConditionID::NormalAttack => true,
                AbilityConditionID::HpFullNormalAttack => {
                    self.combatants[actor].character.hp.current_hp
                        == self.combatants[actor].character.hp.max_hp
                }
                AbilityConditionID::SingleTargetNormalAttack => targets.len() == 1,
                _ => false,
            };
            if valid
                && matches!(
                    a.effect_id,
                    AbilityEffectID::DefenseIgnore | AbilityEffectID::DamageIncrease
                )
                && self.roll(actor, slot)
            {
                self.mark(actor, slot);
                if self.ability(actor, slot).effect_id == AbilityEffectID::DefenseIgnore {
                    context.ignore_defense = true;
                } else {
                    context.damage_multiplier = ability::correction(self.ability(actor, slot));
                }
            }
        }
        for target in targets {
            self.normal_hit(context, target);
        }
        if let Some(i) = cover {
            self.incapacitated(i);
        }
        self.incapacitated(actor);
    }

    fn priority_target(&mut self, actor: usize) -> Option<usize> {
        for slot in 0..self.combatants[actor].character.ability_states.len() {
            let a = self.ability(actor, slot);
            if !self.eligible(actor, slot) {
                continue;
            }
            if !matches!(
                a.effect_id,
                AbilityEffectID::TargetHpLowPriority | AbilityEffectID::TargetDefenseDownPriority
            ) {
                continue;
            }
            let mut candidates: Vec<_> = (0..self.combatants.len())
                .filter(|&i| {
                    self.alive(i)
                        && self.combatants[i].side != self.combatants[actor].side
                        && match a.effect_id {
                            AbilityEffectID::TargetHpLowPriority => {
                                self.hp_threshold(i, a.activation_condition.condition_value)
                            }
                            _ => {
                                self.combatants[i]
                                    .character
                                    .buff_debuff_effect
                                    .defense_debuff
                                    > 0.0
                            }
                        }
                })
                .collect();
            if candidates.is_empty() {
                continue;
            }
            if !self.roll(actor, slot) {
                return None;
            }
            self.mark(actor, slot);
            candidates.sort_by_key(|&i| self.combatants[i].position);
            shuffle(&mut candidates, &mut self.random);
            return candidates.first().copied();
        }
        None
    }

    fn normal_hit(&mut self, context: AttackContext, target: usize) {
        let actor = context.actor;
        let recipient = context.cover.unwrap_or(target);
        if context.cover.is_none() && self.avoided(actor, target) {
            self.events.push(Event::Avoided { actor, target });
            self.counter(actor, target);
            self.incapacitated(target);
            return;
        }
        let blind_miss = status_abnormality::contains(
            &self.combatants[actor].character,
            StatusAbnormalityID::Blindness,
        ) && !self
            .random
            .probability(status_abnormality::BLINDNESS_HIT_RATE);
        if blind_miss {
            if context.cover.is_none() {
                self.counter(actor, target);
                self.incapacitated(target);
            }
            return;
        }
        if context.cover.is_none()
            && !self.counter_used
            && self
                .activate_effect(target, AbilityEffectID::AvoidanceCounter)
                .is_some()
        {
            self.counter_used = true;
            self.events.push(Event::Avoided { actor, target });
            self.secondary_damage(target, actor, actor);
            self.incapacitated(target);
            return;
        }
        self.attack_status(actor, recipient, context.cover.is_some());
        let defense = if context.ignore_defense {
            0.0
        } else {
            self.stat(target, false)
        };
        let mut damage = skill::normal_damage(
            self.stat(actor, true),
            defense,
            context.damage_multiplier,
            &mut self.random,
        );
        if self.erase(recipient) {
            damage = 0.0;
        }
        self.apply_damage(actor, recipient, damage);
        let pursuit =
            self.choose_effect(actor, AbilityEffectID::Pursuit, true, |_, i, _| i != actor);
        if let Some((i, _)) = pursuit {
            self.secondary_damage(i, target, recipient);
        } else if context.cover.is_none() {
            self.counter(actor, target);
        }
        if context.cover.is_none() {
            self.incapacitated(target);
        }
    }

    fn avoided(&mut self, actor: usize, target: usize) -> bool {
        if let Some(slot) = self.slot(target, AbilityEffectID::Avoidance) {
            let normal = matches!(&self.ability(target, slot).effect_data, Some(AbilityMasterDataEffectData::StatusAbnormality(v)) if v.avoidance_type == Some(AbilityAvoidanceType::NormalAttack));
            if normal
                && self.roll(target, slot)
                && self
                    .activate_effect(actor, AbilityEffectID::AvoidanceDisable)
                    .is_none()
            {
                self.mark(target, slot);
                return true;
            }
        }
        self.has_special(target, TacticsBattleSpecialType::Elysion)
    }

    fn counter(&mut self, actor: usize, target: usize) {
        if self.counter_used {
            return;
        }
        if let Some(slot) = self.slot(target, AbilityEffectID::Counter)
            && self.roll(target, slot)
            && self
                .activate_effect(actor, AbilityEffectID::CounterDisable)
                .is_none()
        {
            self.mark(target, slot);
            self.counter_used = true;
            self.secondary_damage(target, actor, actor);
        }
    }

    fn secondary_damage(&mut self, actor: usize, original: usize, recipient: usize) {
        let damage = skill::normal_damage(
            self.stat(actor, true),
            self.stat(original, false),
            1.0,
            &mut self.random,
        );
        self.apply_damage(actor, recipient, damage);
    }

    fn attack_status(&mut self, actor: usize, target: usize, covered: bool) {
        let Some(slot) = self.activate_effect(actor, AbilityEffectID::StatusAbnormalityAttack)
        else {
            return;
        };
        let Some(AbilityMasterDataEffectData::StatusAbnormality(data)) =
            &self.ability(actor, slot).effect_data
        else {
            unreachable!()
        };
        let status = data.status.expect("validated status attack");
        if !covered && let Some(slot) = self.slot(target, AbilityEffectID::Avoidance) {
            let matches = matches!(&self.ability(target, slot).effect_data, Some(AbilityMasterDataEffectData::StatusAbnormality(v)) if v.avoidance_type == Some(AbilityAvoidanceType::StatusAbnormality) && v.status == Some(status));
            if matches && self.roll(target, slot) {
                self.mark(target, slot);
                return;
            }
        }
        self.add_status(actor, target, status);
    }

    fn add_status(&mut self, actor: usize, target: usize, status: StatusAbnormalityID) {
        status_abnormality::apply(&mut self.combatants[target].character, status);
        self.events.push(Event::Status {
            actor,
            target,
            status,
        });
    }
}
