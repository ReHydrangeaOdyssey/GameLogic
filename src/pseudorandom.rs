use crate::Error;

const MULTIPLIER: u64 = 6_364_136_223_846_793_005;
const INCREMENT: u64 = 1_442_695_040_888_963_407;
const PRECISION: u32 = 10_000;
const DAMAGE_STEPS: u32 = 301;
pub const TIME_SEED: u64 = 202_205_311_459;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Random {
    state: u64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        let mut random = Self { state: 0 };
        random.next_u32();
        random.state = random.state.wrapping_add(seed);
        random.next_u32();
        random
    }

    pub fn from_state(state: u64) -> Self {
        Self { state }
    }
    pub fn state(&self) -> u64 {
        self.state
    }

    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        // PCG: state' = state * multiplier + increment modulo 2^64.
        self.state = old_state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rotation = (old_state >> 59) as u32;
        xorshifted.rotate_right(rotation)
    }

    pub fn next_bounded(&mut self, bound: u32) -> u32 {
        if bound == 0 {
            return 0;
        }
        // Reject the incomplete residue range to avoid modulo bias.
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let value = self.next_u32();
            if value >= threshold {
                return value % bound;
            }
        }
    }

    pub fn probability(&mut self, probability: f32) -> bool {
        // p = next_bounded(10000) / 10000; success iff probability > p.
        probability > self.next_bounded(PRECISION) as f32 / PRECISION as f32
    }

    pub fn damage_multiplier(&mut self) -> f32 {
        // Inclusive 1.0000 .. 1.0300 in steps of 0.0001.
        1.0 + self.next_bounded(DAMAGE_STEPS) as f32 * 0.0001
    }
}

pub fn time_based_seed(epoch_microseconds: u64) -> u64 {
    // The caller supplies time; game-core never reads a system clock.
    TIME_SEED ^ epoch_microseconds
}

pub fn shuffle<T>(values: &mut [T], random: &mut Random) {
    assert!(
        u32::try_from(values.len()).is_ok(),
        "shuffle length exceeds u32"
    );
    for i in (1..values.len()).rev() {
        let j = random.next_bounded((i + 1) as u32) as usize;
        values.swap(i, j);
    }
}

pub fn weighted_shuffle<T>(
    values: &mut [T],
    weights: &mut [u32],
    random: &mut Random,
) -> Result<(), Error> {
    if values.len() != weights.len() || weights.contains(&0) {
        return Err(Error::InvalidValue("weights"));
    }
    let mut total = weights
        .iter()
        .try_fold(0_u32, |sum, w| sum.checked_add(*w))
        .ok_or(Error::ArithmeticOverflow)?;
    for i in 0..values.len() {
        let mut value = random.next_bounded(total);
        let mut j = i;
        while value >= weights[j] {
            value -= weights[j];
            j += 1;
        }
        values.swap(i, j);
        weights.swap(i, j);
        total -= weights[i];
    }
    Ok(())
}
