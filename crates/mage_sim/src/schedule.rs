//! The tick chain: every simulation system runs in exactly one of these sets in `FixedUpdate`,
//! and the sets run in this order every tick (simulationsmodell.md, invariant 2).

use bevy::ecs::prelude::SystemSet;

/// The sets of the tick chain, in order. German names in the vault: Intent, Werte, Casting,
/// Bewegung, Physik, Treffer, Schaden, Sichtbarkeit, Aufräumen.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SimSet {
    /// Freeze each unit's intent for this tick.
    Intent,
    /// Count down attribute modifiers, recompute effective attributes.
    Values,
    /// Cooldowns, mana, casts.
    Casting,
    /// Move units and spells.
    Movement,
    /// Avian's step: brings colliders up to this tick's positions.
    Physics,
    /// Shape casts and fuses; impacts are only collected.
    Hits,
    /// Sorted impacts, effects, damage, death.
    Damage,
    /// Who sees what.
    Visibility,
    /// Despawn spent spells, respawn.
    Cleanup,
}
