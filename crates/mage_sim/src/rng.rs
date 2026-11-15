//! The one source of randomness in the simulation (ADR-0009). Own code instead of a library:
//! xoshiro256** seeded with SplitMix64, integer operations with defined overflow only, so the
//! same seed draws the same numbers on every platform and compiler.

use bevy::ecs::prelude::{Commands, Res, Resource};

use crate::scenario::Scenario;

/// The generator. Simulation state: part of every snapshot.
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct SimRng {
    state: [u64; 4],
}

impl SimRng {
    /// Expands `seed` into the generator state with SplitMix64.
    pub fn from_seed(seed: u64) -> Self {
        let mut mix = seed;
        let mut next = || {
            mix = mix.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = mix;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        Self {
            state: [next(), next(), next(), next()],
        }
    }

    /// The next 64 random bits (xoshiro256**).
    pub fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = &mut self.state;
        let result = s1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = *s1 << 17;
        *s2 ^= *s0;
        *s3 ^= *s1;
        *s1 ^= *s2;
        *s0 ^= *s3;
        *s2 ^= t;
        *s3 = s3.rotate_left(45);
        result
    }

    /// A uniformly distributed integer in `min..=max`, without modulo bias.
    pub fn range_inclusive(&mut self, min: i64, max: i64) -> i64 {
        assert!(min <= max, "empty range {min}..={max}");
        let span = max.abs_diff(min).wrapping_add(1);
        if span == 0 {
            // The full range of i64.
            return self.next_u64() as i64;
        }
        let limit = u64::MAX - u64::MAX % span;
        loop {
            let draw = self.next_u64();
            if draw < limit {
                return min.wrapping_add((draw % span) as i64);
            }
        }
    }
}

/// Seeds the generator from the scenario at startup.
pub fn seed_rng(mut commands: Commands, scenario: Res<Scenario>) {
    commands.insert_resource(SimRng::from_seed(scenario.seed()));
}
