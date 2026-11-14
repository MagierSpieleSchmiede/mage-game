//! B-006: intents are frozen per unit and tick from the input buffer; a press is never lost and
//! never counted twice, however many ticks a frame runs (intent-steuerung.md, ADR-0004,
//! ADR-0030, ADR-0031).

use bevy::app::{App, FixedUpdate};
use bevy::ecs::prelude::{IntoScheduleConfigs, Mut, Query, ResMut, Resource};
use bevy::math::{Vec2, Vec3};
use bevy::time::TimeUpdateStrategy;
use mage_sim::headless::{headless_app, step_ticks};
use mage_sim::ids::{PlayerId, SimId};
use mage_sim::intent::{Controller, InputBuffer, IntentForTick, MAX_SLOTS};
use mage_sim::scenario::Scenario;
use mage_sim::schedule::SimSet;

const ME: SimId = SimId(0);
const OTHER: SimId = SimId(1);

/// What unit `ME` saw in the set Casting, one entry per tick.
#[derive(Resource, Default)]
struct Seen(Vec<IntentForTick>);

fn record(units: Query<(&SimId, &IntentForTick)>, mut seen: ResMut<Seen>) {
    for (sim_id, intent) in &units {
        if *sim_id == ME {
            seen.0.push(intent.clone());
        }
    }
}

fn app() -> App {
    let mut app = headless_app(Scenario::builder().team(1).team(1).build().unwrap());
    app.init_resource::<Seen>()
        .add_systems(FixedUpdate, record.in_set(SimSet::Casting));
    app
}

fn buffer(app: &mut App) -> Mut<'_, InputBuffer> {
    app.world_mut().resource_mut::<InputBuffer>()
}

/// One frame that runs `ticks` fixed ticks.
fn frame(app: &mut App, ticks: u32) {
    app.insert_resource(TimeUpdateStrategy::FixedTimesteps(ticks));
    app.update();
    app.insert_resource(TimeUpdateStrategy::FixedTimesteps(1));
}

fn pressed(app: &App, slot: usize) -> Vec<bool> {
    app.world()
        .resource::<Seen>()
        .0
        .iter()
        .map(|i| i.intent.pressed[slot])
        .collect()
}

#[test]
fn there_are_twelve_slots() {
    assert_eq!(MAX_SLOTS, 12);
}

#[test]
fn every_mage_is_steered_by_its_player() {
    let mut app = app();
    let world = app.world_mut();
    let mut query = world.query::<(&PlayerId, &Controller)>();
    let mut found = 0;
    for (player, controller) in query.iter(world) {
        assert_eq!(*controller, Controller::Player(*player));
        found += 1;
    }
    assert_eq!(found, 2);
}

#[test]
fn a_press_survives_a_frame_without_a_tick() {
    let mut app = app();
    buffer(&mut app).entry(ME).press(2);
    frame(&mut app, 0);
    assert!(pressed(&app, 2).is_empty());
    frame(&mut app, 1);
    assert_eq!(pressed(&app, 2), [true]);
}

#[test]
fn a_press_counts_only_in_the_first_of_several_ticks_of_a_frame() {
    let mut app = app();
    buffer(&mut app).entry(ME).press(0);
    frame(&mut app, 3);
    assert_eq!(pressed(&app, 0), [true, false, false]);
}

#[test]
fn two_presses_between_ticks_are_one_pressed_tick() {
    let mut app = app();
    buffer(&mut app).entry(ME).press(1);
    buffer(&mut app).entry(ME).press(1);
    step_ticks(&mut app, 2);
    assert_eq!(pressed(&app, 1), [true, false]);
}

#[test]
fn presses_in_successive_frames_are_successive_pressed_ticks() {
    let mut app = app();
    for _ in 0..3 {
        buffer(&mut app).entry(ME).press(4);
        step_ticks(&mut app, 1);
    }
    assert_eq!(pressed(&app, 4), [true, true, true]);
}

#[test]
fn just_released_follows_held() {
    let mut app = app();
    buffer(&mut app).entry(ME).set_held(5, true);
    step_ticks(&mut app, 1);
    buffer(&mut app).entry(ME).set_held(5, false);
    step_ticks(&mut app, 2);
    let seen = &app.world().resource::<Seen>().0;
    let held: Vec<bool> = seen.iter().map(|i| i.intent.held[5]).collect();
    let released: Vec<bool> = seen.iter().map(|i| i.just_released[5]).collect();
    assert_eq!(held, [true, false, false]);
    assert_eq!(released, [false, true, false]);
}

#[test]
fn an_intent_only_reaches_its_own_unit() {
    let mut app = app();
    buffer(&mut app).entry(ME).press(0);
    buffer(&mut app).entry(ME).move_axis = Vec2::new(1.0, 0.0);
    step_ticks(&mut app, 1);
    let world = app.world_mut();
    let mut query = world.query::<(&SimId, &IntentForTick)>();
    for (sim_id, intent) in query.iter(world) {
        if *sim_id == OTHER {
            assert!(!intent.intent.pressed.iter().any(|&p| p));
            assert_eq!(intent.intent.move_axis, Vec2::ZERO);
        }
    }
}

#[test]
fn slots_beyond_the_bar_and_unknown_units_are_ignored() {
    let mut app = app();
    buffer(&mut app).entry(ME).press(MAX_SLOTS);
    buffer(&mut app).entry(ME).set_held(MAX_SLOTS + 3, true);
    buffer(&mut app).entry(SimId(999)).press(0);
    step_ticks(&mut app, 1);
    let intent = &app.world().resource::<Seen>().0[0].intent;
    assert!(!intent.pressed.iter().any(|&p| p));
    assert!(!intent.held.iter().any(|&h| h));
}

#[test]
fn movement_is_clamped_and_nonsense_is_dropped() {
    let mut app = app();
    buffer(&mut app).entry(ME).move_axis = Vec2::new(3.0, 4.0);
    step_ticks(&mut app, 1);
    buffer(&mut app).entry(ME).move_axis = Vec2::new(f32::NAN, 1.0);
    step_ticks(&mut app, 1);
    let seen = &app.world().resource::<Seen>().0;
    assert!((seen[0].intent.move_axis - Vec2::new(0.6, 0.8)).length() < 1e-6);
    assert_eq!(seen[1].intent.move_axis, Vec2::ZERO);
}

#[test]
fn aim_is_normalised_and_an_invalid_aim_keeps_the_previous_one() {
    let mut app = app();
    buffer(&mut app).entry(ME).aim_dir = Vec3::new(0.0, 0.0, 5.0);
    step_ticks(&mut app, 1);
    buffer(&mut app).entry(ME).aim_dir = Vec3::ZERO;
    step_ticks(&mut app, 1);
    buffer(&mut app).entry(ME).aim_dir = Vec3::new(f32::INFINITY, 0.0, 0.0);
    step_ticks(&mut app, 1);
    let aims: Vec<Vec3> = app
        .world()
        .resource::<Seen>()
        .0
        .iter()
        .map(|i| i.intent.aim_dir)
        .collect();
    assert_eq!(aims, [Vec3::Z, Vec3::Z, Vec3::Z]);
}
