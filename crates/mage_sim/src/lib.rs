//! The simulation of Mage Game: all gameplay as a deterministic tick chain in `FixedUpdate`.
//!
//! This crate runs headless and must build for every target platform, `wasm32-unknown-unknown`
//! included, so it depends on Bevy and Avian without windowing, rendering, audio or threads
//! (ADR-0020, ADR-0024). The test `tests/no_render_dependencies.rs` enforces this.

use bevy::app::{App, Plugin, Startup};
use bevy::ecs::prelude::{IntoScheduleConfigs, resource_exists};
use bevy::time::{Fixed, Time};

use crate::ids::SimIdCounter;
use crate::scenario::Scenario;

pub mod headless;
pub mod ids;
pub mod mage;
pub mod scenario;

/// Simulation ticks per second, on every machine and on the server (ADR-0001, ADR-0008).
pub const TICKS_PER_SECOND: u32 = 60;

/// The simulation. Expects Bevy's `TimePlugin`, which the client gets from its default
/// plugins and [`headless::headless_app`] adds itself. If a [`Scenario`] is present at
/// startup, its mages are spawned.
pub struct MageSimPlugin;

impl Plugin for MageSimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(f64::from(TICKS_PER_SECOND)))
            .init_resource::<SimIdCounter>()
            .add_systems(
                Startup,
                mage::spawn_scenario_mages.run_if(resource_exists::<Scenario>),
            );
    }
}
