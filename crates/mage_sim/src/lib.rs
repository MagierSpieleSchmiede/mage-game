//! The simulation of Mage Game: all gameplay as a deterministic tick chain in `FixedUpdate`.
//!
//! This crate runs headless and must build for every target platform, `wasm32-unknown-unknown`
//! included, so it depends on Bevy and Avian without windowing, rendering, audio or threads
//! (ADR-0020, ADR-0024). The test `tests/no_render_dependencies.rs` enforces this.

use avian3d::prelude::{PhysicsPlugins, PhysicsSystems, SpatialQuerySystems};
use bevy::app::{App, FixedUpdate, Plugin, Startup};
use bevy::ecs::prelude::{IntoScheduleConfigs, resource_exists};
use bevy::time::{Fixed, Time};

use crate::ids::SimIdCounter;
use crate::scenario::Scenario;
use crate::schedule::SimSet;

pub mod headless;
pub mod ids;
pub mod mage;
pub mod scenario;
pub mod schedule;

/// Simulation ticks per second, on every machine and on the server (ADR-0001, ADR-0008).
pub const TICKS_PER_SECOND: u32 = 60;

/// The simulation. Expects Bevy's `TimePlugin`, which the client gets from its default
/// plugins and [`headless::headless_app`] adds itself. If a [`Scenario`] is present at
/// startup, its mages are spawned.
pub struct MageSimPlugin;

impl Plugin for MageSimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(f64::from(TICKS_PER_SECOND)))
            // Avian steps inside the set Physics of the chain, not in its default
            // `FixedPostUpdate`, so the set Hits casts against this tick's positions (P-005).
            .add_plugins(PhysicsPlugins::new(FixedUpdate))
            .configure_sets(
                FixedUpdate,
                (
                    SimSet::Intent,
                    SimSet::Values,
                    SimSet::Casting,
                    SimSet::Movement,
                    SimSet::Physics,
                    SimSet::Hits,
                    SimSet::Damage,
                    SimSet::Visibility,
                    SimSet::Cleanup,
                )
                    .chain(),
            )
            .configure_sets(
                FixedUpdate,
                (
                    PhysicsSystems::First,
                    PhysicsSystems::Prepare,
                    PhysicsSystems::StepSimulation,
                    PhysicsSystems::Writeback,
                    PhysicsSystems::Last,
                    SpatialQuerySystems,
                )
                    .in_set(SimSet::Physics),
            )
            .init_resource::<SimIdCounter>()
            .add_systems(
                Startup,
                mage::spawn_scenario_mages.run_if(resource_exists::<Scenario>),
            );
    }
}
