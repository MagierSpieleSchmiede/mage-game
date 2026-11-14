//! B-004: every simulation entity gets a `SimId` from one counter; the scenario spawns one
//! mage per player, with its `PlayerId` and `TeamId` (serialisierung-ids.md, teams.md).

use bevy::app::App;
use bevy::ecs::prelude::With;
use mage_sim::headless::{headless_app, step_ticks};
use mage_sim::ids::{PlayerId, SimId, SimIdCounter, TeamId};
use mage_sim::mage::Mage;
use mage_sim::scenario::Scenario;

/// `(SimId, PlayerId, TeamId)` of every mage, ordered by `SimId`.
fn mages(app: &mut App) -> Vec<(SimId, PlayerId, TeamId)> {
    let world = app.world_mut();
    let mut query = world.query_filtered::<(&SimId, &PlayerId, &TeamId), With<Mage>>();
    let mut mages: Vec<_> = query.iter(world).map(|(s, p, t)| (*s, *p, *t)).collect();
    mages.sort_by_key(|(sim_id, _, _)| *sim_id);
    mages
}

fn app_for(team_sizes: &[u8]) -> App {
    let mut builder = Scenario::builder().seed(3);
    for &size in team_sizes {
        builder = builder.team(size);
    }
    headless_app(builder.build().unwrap())
}

#[test]
fn the_counter_hands_out_consecutive_ids() {
    let mut counter = SimIdCounter::default();
    assert_eq!(counter.allocate(), SimId(0));
    assert_eq!(counter.allocate(), SimId(1));
    assert_eq!(counter.allocate(), SimId(2));
}

#[test]
fn two_vs_two_has_four_mages_in_two_teams() {
    let mut app = app_for(&[2, 2]);
    let teams: Vec<TeamId> = mages(&mut app).iter().map(|(_, _, t)| *t).collect();
    assert_eq!(teams, [TeamId(0), TeamId(0), TeamId(1), TeamId(1)]);
}

#[test]
fn sim_ids_are_consecutive_and_unique() {
    let mut app = app_for(&[3, 1, 2]);
    let sim_ids: Vec<SimId> = mages(&mut app).iter().map(|(s, _, _)| *s).collect();
    assert_eq!(sim_ids, (0..6).map(SimId).collect::<Vec<_>>());
    // The next entity continues the sequence.
    let next = app.world_mut().resource_mut::<SimIdCounter>().allocate();
    assert_eq!(next, SimId(6));
}

#[test]
fn players_are_numbered_in_team_order() {
    let mut app = app_for(&[3, 1, 2]);
    let players: Vec<(PlayerId, TeamId)> =
        mages(&mut app).iter().map(|(_, p, t)| (*p, *t)).collect();
    assert_eq!(
        players,
        [
            (PlayerId(0), TeamId(0)),
            (PlayerId(1), TeamId(0)),
            (PlayerId(2), TeamId(0)),
            (PlayerId(3), TeamId(1)),
            (PlayerId(4), TeamId(2)),
            (PlayerId(5), TeamId(2)),
        ]
    );
}

#[test]
fn mages_exist_before_the_first_tick_and_are_spawned_once() {
    let mut app = app_for(&[1, 1]);
    assert_eq!(mages(&mut app).len(), 2);
    step_ticks(&mut app, 5);
    assert_eq!(mages(&mut app).len(), 2);
}

#[test]
fn the_same_scenario_spawns_the_same_mages() {
    let mut first = app_for(&[2, 3]);
    let mut second = app_for(&[2, 3]);
    assert_eq!(mages(&mut first), mages(&mut second));
}
