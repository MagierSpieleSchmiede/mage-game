//! The core every unit shares, mage or creature (einheiten.md, ADR-0031).

use bevy::ecs::prelude::Component;

use crate::attributes::{EffectiveAttributes, Modifiers};

/// Marks a unit: anything that can fight. Brings the parts of the core that start empty;
/// the spawner adds `SimId`, `TeamId`, `BaseAttributes` and a `Controller`.
#[derive(Component, Debug, Default)]
#[require(Modifiers, EffectiveAttributes)]
pub struct Unit;
