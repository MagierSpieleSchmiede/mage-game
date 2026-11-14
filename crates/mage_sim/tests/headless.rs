//! B-003: the headless app ticks exactly as often as asked, without reading a clock, and
//! scenarios describe teams, players and seed (headless-integrationstest.md, teams.md).

use std::time::Duration;

use bevy::app::{App, FixedUpdate};
use bevy::ecs::prelude::{ResMut, Resource};
use bevy::time::{Fixed, Time};
use mage_sim::TICKS_PER_SECOND;
use mage_sim::headless::{headless_app, step_ticks};
use mage_sim::scenario::{MAX_TEAMS, Scenario, ScenarioError};

/// Counts how often `FixedUpdate` ran. Test-only probe, not part of the simulation.
#[derive(Resource, Default)]
struct FixedTicks(u32);

fn count_tick(mut ticks: ResMut<FixedTicks>) {
    ticks.0 += 1;
}

fn duel() -> Scenario {
    Scenario::builder().team(1).team(1).seed(1).build().unwrap()
}

fn probed_app() -> App {
    let mut app = headless_app(duel());
    app.init_resource::<FixedTicks>()
        .add_systems(FixedUpdate, count_tick);
    app
}

fn ticks(app: &App) -> u32 {
    app.world().resource::<FixedTicks>().0
}

#[test]
fn a_new_headless_app_has_not_ticked() {
    let app = probed_app();
    assert_eq!(ticks(&app), 0);
}

#[test]
fn step_ticks_runs_exactly_n_fixed_ticks() {
    let mut app = probed_app();
    let mut expected = 0;
    for n in [0, 1, 1, 7, 60, 600] {
        step_ticks(&mut app, n);
        expected += n;
        assert_eq!(ticks(&app), expected, "after step_ticks({n})");
    }
}

#[test]
fn a_tick_is_one_sixtieth_of_a_second() {
    let app = headless_app(duel());
    let timestep = app.world().resource::<Time<Fixed>>().timestep();
    assert_eq!(TICKS_PER_SECOND, 60);
    assert_eq!(
        timestep,
        Duration::from_secs_f64(1.0 / f64::from(TICKS_PER_SECOND))
    );
}

#[test]
fn the_app_carries_its_scenario() {
    let scenario = Scenario::builder()
        .team(2)
        .team(2)
        .seed(42)
        .build()
        .unwrap();
    let app = headless_app(scenario.clone());
    assert_eq!(app.world().resource::<Scenario>(), &scenario);
}

#[test]
fn a_scenario_has_teams_of_players_and_a_seed() {
    let scenario = Scenario::builder()
        .team(3)
        .team(3)
        .team(1)
        .seed(7)
        .build()
        .unwrap();
    assert_eq!(scenario.team_sizes(), &[3, 3, 1]);
    assert_eq!(scenario.player_count(), 7);
    assert_eq!(scenario.seed(), 7);
}

#[test]
fn a_scenario_needs_at_least_two_teams() {
    assert_eq!(
        Scenario::builder().team(4).build(),
        Err(ScenarioError::TooFewTeams)
    );
}

#[test]
fn every_team_needs_a_player() {
    assert_eq!(
        Scenario::builder().team(1).team(0).build(),
        Err(ScenarioError::EmptyTeam)
    );
}

#[test]
fn teams_are_limited_by_the_visibility_mask() {
    let mut builder = Scenario::builder();
    for _ in 0..MAX_TEAMS {
        builder = builder.team(1);
    }
    assert!(builder.clone().build().is_ok());
    assert_eq!(builder.team(1).build(), Err(ScenarioError::TooManyTeams));
}
