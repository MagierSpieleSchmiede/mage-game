//! The mage: one entity per player, spawned from the scenario.

use bevy::ecs::prelude::{Commands, Component, Res, ResMut};

use crate::ids::{PlayerId, SimIdCounter, TeamId};
use crate::intent::Controller;
use crate::scenario::Scenario;

/// Marks a mage. Carries no visuals; the presentation reacts to it (invariant 6).
#[derive(Component, Debug, Default)]
pub struct Mage;

/// Spawns one mage per player of the scenario, team by team. Players are numbered in that
/// order, starting at 0, so the same scenario always gives the same ids.
pub fn spawn_scenario_mages(
    mut commands: Commands,
    scenario: Res<Scenario>,
    mut sim_ids: ResMut<SimIdCounter>,
) {
    let mut next_player = 0;
    for (team, &size) in scenario.team_sizes().iter().enumerate() {
        let team = TeamId(u8::try_from(team).expect("a scenario has at most 32 teams"));
        for _ in 0..size {
            let player = PlayerId(next_player);
            commands.spawn((
                Mage,
                sim_ids.allocate(),
                player,
                team,
                Controller::Player(player),
            ));
            next_player += 1;
        }
    }
}
