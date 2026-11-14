//! Stable identities. Anything that is stored, compared across runs or sent over a network
//! names entities by these, never by `Entity` (ADR-0013, serialisierung-ids.md).

use bevy::ecs::prelude::{Component, Resource};

/// Identity of a simulation entity, handed out in spawn order by [`SimIdCounter`]. Decides
/// every order that affects the outcome. 64 bits, because in a large world a long-running
/// server spawns and streams in entities without end (ADR-0028).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimId(pub u64);

/// The one source of [`SimId`]s. Part of the simulation state.
#[derive(Resource, Debug, Default)]
pub struct SimIdCounter {
    next: u64,
}

impl SimIdCounter {
    /// The next unused id.
    pub fn allocate(&mut self) -> SimId {
        let id = SimId(self.next);
        self.next += 1;
        id
    }
}

/// Who controls a mage. 32 bits, room for thousands of players in a large world.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerId(pub u32);

/// Which team a mage belongs to; "enemy" always means another `TeamId` (ADR-0015).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TeamId(pub u8);

/// Identity of an effect, e.g. Slow; modifiers of the same effect follow its stacking rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectId(pub u16);

/// Which AI behaviour steers a unit (ADR-0031).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BehaviorId(pub u16);

/// Key of a spell blueprint in the registry (zauber-datenmodell.md).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellId(pub u16);
