//! A scenario describes a round: its teams, the players in each team and the seed. Tests,
//! benchmarks, AI training and later the network lobby all start from one (teams.md).

use bevy::ecs::prelude::Resource;

/// At most this many teams, one bit each in the visibility mask (ADR-0015, ADR-0018).
pub const MAX_TEAMS: usize = 32;

/// A valid round setup: two to [`MAX_TEAMS`] teams, each with at least one player.
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct Scenario {
    team_sizes: Vec<u8>,
    seed: u64,
}

/// Why a [`ScenarioBuilder`] refused to build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScenarioError {
    /// A round needs two or more teams.
    TooFewTeams,
    /// More teams than the visibility mask has bits.
    TooManyTeams,
    /// A team without players.
    EmptyTeam,
}

impl Scenario {
    pub fn builder() -> ScenarioBuilder {
        ScenarioBuilder::default()
    }

    /// Players per team, in team order.
    pub fn team_sizes(&self) -> &[u8] {
        &self.team_sizes
    }

    pub fn player_count(&self) -> usize {
        self.team_sizes.iter().map(|&size| usize::from(size)).sum()
    }

    /// Seed of the round's one random number generator (ADR-0009).
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

/// Builds a [`Scenario`] team by team. The seed defaults to 0.
#[derive(Clone, Debug, Default)]
pub struct ScenarioBuilder {
    team_sizes: Vec<u8>,
    seed: u64,
}

impl ScenarioBuilder {
    /// Adds a team with `players` players.
    pub fn team(mut self, players: u8) -> Self {
        self.team_sizes.push(players);
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn build(self) -> Result<Scenario, ScenarioError> {
        if self.team_sizes.len() < 2 {
            return Err(ScenarioError::TooFewTeams);
        }
        if self.team_sizes.len() > MAX_TEAMS {
            return Err(ScenarioError::TooManyTeams);
        }
        if self.team_sizes.contains(&0) {
            return Err(ScenarioError::EmptyTeam);
        }
        Ok(Scenario {
            team_sizes: self.team_sizes,
            seed: self.seed,
        })
    }
}
