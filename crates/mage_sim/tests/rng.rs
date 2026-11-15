//! B-008: the one random number generator of the simulation, seeded from the scenario
//! (ADR-0009, determinismus.md). Its own code, integer-only, so every platform draws the same
//! numbers; the reference values below were computed independently (Python, from the published
//! SplitMix64 and xoshiro256** algorithms).

use mage_sim::headless::headless_app;
use mage_sim::rng::SimRng;
use mage_sim::scenario::Scenario;

fn draws(seed: u64, n: usize) -> Vec<u64> {
    let mut rng = SimRng::from_seed(seed);
    (0..n).map(|_| rng.next_u64()).collect()
}

#[test]
fn draws_match_the_reference_values() {
    assert_eq!(
        draws(0, 4),
        [
            0x99ec_5f36_cb75_f2b4,
            0xbf6e_1f78_4956_452a,
            0x1a5f_849d_4933_e6e0,
            0x6aa5_94f1_262d_2d2c,
        ]
    );
    assert_eq!(
        draws(42, 4),
        [
            0x1578_0b2e_0c2e_c716,
            0x6104_d986_6d11_3a7e,
            0xae17_5332_39e4_99a1,
            0xecb8_ad47_03b3_60a1,
        ]
    );
}

#[test]
fn the_same_seed_gives_the_same_sequence() {
    assert_eq!(draws(7, 1_000), draws(7, 1_000));
}

#[test]
fn different_seeds_give_different_sequences() {
    assert_ne!(draws(7, 16), draws(8, 16));
}

#[test]
fn a_range_stays_inside_and_reaches_both_ends() {
    let mut rng = SimRng::from_seed(3);
    let mut seen = [false; 5];
    for _ in 0..1_000 {
        let value = rng.range_inclusive(-2, 2);
        assert!((-2..=2).contains(&value), "{value}");
        seen[(value + 2) as usize] = true;
    }
    assert_eq!(seen, [true; 5]);
}

#[test]
fn a_range_of_one_value_is_that_value() {
    let mut rng = SimRng::from_seed(3);
    assert_eq!(rng.range_inclusive(9, 9), 9);
}

#[test]
fn the_app_seeds_its_generator_from_the_scenario() {
    let scenario = Scenario::builder()
        .team(1)
        .team(1)
        .seed(42)
        .build()
        .unwrap();
    let app = headless_app(scenario);
    assert_eq!(app.world().resource::<SimRng>(), &SimRng::from_seed(42));
}
