use crate::{model::*, pseudorandom::Random};

pub fn contains(character: &CharacterBattle, status: StatusAbnormalityID) -> bool {
    character
        .status_abnormalities
        .iter()
        .any(|s| s.status_id == status)
}

pub fn apply(character: &mut CharacterBattle, status: StatusAbnormalityID) {
    if let Some(state) = character
        .status_abnormalities
        .iter_mut()
        .find(|s| s.status_id == status)
    {
        state.elapsed_turns = 1;
    } else {
        character.status_abnormalities.push(StatusAbnormalityState {
            status_id: status,
            elapsed_turns: 1,
            poison_cycle_turns: 0,
        });
        character.status_abnormalities.sort_by_key(|s| s.status_id);
    }
}

/// Caller visits characters in (position, player ID, fixed side) order.
/// Returns true when coma is removed, so the scheduler can add the speed wait.
pub fn end_turn(character: &mut CharacterBattle, random: &mut Random) -> bool {
    let mut woke = false;
    character.status_abnormalities.sort_by_key(|s| s.status_id);
    character.status_abnormalities.retain_mut(|state| {
        if state.status_id == StatusAbnormalityID::Poison {
            state.poison_cycle_turns += 1;
            if state.poison_cycle_turns == 3 {
                // Poison every third turn: HP' = max(HP - ceil(HP * 0.05), 0).
                character.hp.current_hp =
                    (character.hp.current_hp - (character.hp.current_hp * 0.05).ceil()).max(0.0);
                state.poison_cycle_turns = 0;
            }
        } else {
            let minimum = if state.status_id == StatusAbnormalityID::Coma {
                3
            } else {
                5
            };
            if state.elapsed_turns >= minimum && random.probability(0.5) {
                woke |= state.status_id == StatusAbnormalityID::Coma;
                return false;
            }
        }
        state.elapsed_turns = state.elapsed_turns.saturating_add(1);
        true
    });
    woke
}

pub fn update_buff_state(character: &mut CharacterBattle) {
    let b = &character.buff_debuff_effect;
    character.buff_debuff_state = match (
        b.attack_buff != 0.0 || b.defense_buff != 0.0,
        b.attack_debuff != 0.0 || b.defense_debuff != 0.0,
    ) {
        (false, false) => BuffDebuffState::None,
        (true, false) => BuffDebuffState::Buff,
        (false, true) => BuffDebuffState::Debuff,
        (true, true) => BuffDebuffState::BuffDebuff,
    };
}

impl HitPoints {
    pub fn damage(&mut self, damage: f32) {
        // Damage is floored only at HP application; initial fractional HP is retained.
        self.current_hp = (self.current_hp - damage.floor()).max(0.0);
    }
    pub fn heal(&mut self, amount: f32) {
        // Recovery retains fractional values and cannot exceed max HP.
        self.current_hp = (self.current_hp + amount).clamp(0.0, self.max_hp);
    }
}
