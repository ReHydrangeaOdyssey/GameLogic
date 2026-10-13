use super::Battle;
use crate::pseudorandom::shuffle;

impl Battle<'_> {
    pub(super) fn speed(&self, i: usize) -> u32 {
        let c = &self.combatants[i];
        // Speed level = clamp(base level + formation + all tactics speed corrections, 0, 32).
        (c.character.speed as u32 as f32
            + c.formation.speed
            + self.tactics_correction(i, super::tactical::Stat::Speed))
        .clamp(0.0, 32.0) as u32
    }

    pub(super) fn sorted(&mut self, mut indices: Vec<usize>, speed_first: bool) -> Vec<usize> {
        let key = |i: usize| {
            let c = &self.combatants[i];
            if speed_first {
                (self.speed(i), c.position.value() as u32)
            } else {
                (c.position.value() as u32, self.speed(i))
            }
        };
        indices.sort_by_key(|&i| {
            (
                key(i),
                self.combatants[i].character.owner_player_id,
                self.combatants[i].side,
            )
        });
        let keys: Vec<_> = indices.iter().map(|&i| key(i)).collect();
        let mut start = 0;
        while start < indices.len() {
            let mut end = start + 1;
            while end < indices.len() && keys[end] == keys[start] {
                end += 1;
            }
            shuffle(&mut indices[start..end], &mut self.random);
            start = end;
        }
        indices
    }

    pub(super) fn initial_order(&mut self) {
        let indices = (0..self.combatants.len())
            .filter(|&i| self.alive(i))
            .collect();
        let indices = self.sorted(indices, true);
        for (index, i) in indices.into_iter().enumerate() {
            if index < 5 {
                self.attack_queue.push_back(i);
            } else {
                self.wait[i] += self.speed(i);
            }
        }
        self.filter_queues();
    }

    pub(super) fn filter_queues(&mut self) {
        let actable: Vec<_> = (0..self.combatants.len())
            .map(|i| self.actable(i))
            .collect();
        self.attack_queue.retain(|&i| actable[i]);
        self.wait_queue.retain(|&i| actable[i]);
    }

    pub(super) fn refill(&mut self) {
        self.filter_queues();
        if self.attack_queue.len() == 5 {
            return;
        }
        let had_waiters = !self.wait_queue.is_empty();
        while self.attack_queue.len() < 5 {
            if let Some(i) = self.wait_queue.pop_front() {
                self.attack_queue.push_back(i);
            } else {
                break;
            }
        }
        if had_waiters || self.attack_queue.len() == 5 {
            return;
        }
        let indices = (0..self.combatants.len())
            .filter(|&i| {
                self.actable(i)
                    && self.wait[i] == 0
                    && !self.attack_queue.contains(&i)
                    && !self.wait_queue.contains(&i)
            })
            .collect();
        for i in self.sorted(indices, false) {
            if self.attack_queue.len() < 5 {
                self.attack_queue.push_back(i);
            } else {
                self.wait_queue.push_back(i);
            }
        }
    }

    pub(super) fn update_wait(&mut self) {
        self.filter_queues();
        let decrement = if self.attack_queue.is_empty() && self.wait_queue.is_empty() {
            (0..self.combatants.len())
                .filter(|&i| self.actable(i))
                .map(|i| self.wait[i])
                .min()
                .unwrap_or(0)
        } else {
            1
        };
        for i in 0..self.wait.len() {
            if self.alive(i) && self.wait[i] > 0 {
                self.wait[i] = self.wait[i].saturating_sub(decrement);
            }
        }
    }

    pub(super) fn after_action_wait(&mut self, actor: usize) {
        let living: Vec<_> = (0..self.combatants.len())
            .filter(|&i| self.alive(i))
            .collect();
        let mean = if self.wait_queue.is_empty() && !living.is_empty() {
            // Integer division floors the mean speed level of living characters.
            living.iter().map(|&i| self.speed(i)).sum::<u32>() / living.len() as u32
        } else {
            0
        };
        self.wait[actor] += self.speed(actor) + mean;
    }
}
