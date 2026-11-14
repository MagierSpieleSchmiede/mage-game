//! The simulation without window or renderer, driven tick by tick: the normal case for tests,
//! benchmarks, the server and RL training (headless-integrationstest.md). Lives beside the
//! simulation plugins, not in them.

use bevy::app::App;
use bevy::time::{TimePlugin, TimeUpdateStrategy};

use crate::MageSimPlugin;
use crate::scenario::Scenario;

/// An app of only the simulation, for `scenario`, that has not run a tick yet.
///
/// Time never comes from a clock: every `App::update` advances it by exactly one fixed
/// timestep, so it runs `FixedUpdate` exactly once. The very first update only starts the
/// clock and runs no tick, so it happens here, before the app is handed out.
pub fn headless_app(scenario: Scenario) -> App {
    let mut app = App::new();
    app.add_plugins((TimePlugin, MageSimPlugin))
        .insert_resource(TimeUpdateStrategy::FixedTimesteps(1))
        .insert_resource(scenario);
    app.update();
    app
}

/// Runs exactly `n` simulation ticks.
pub fn step_ticks(app: &mut App, n: u32) {
    for _ in 0..n {
        app.update();
    }
}
