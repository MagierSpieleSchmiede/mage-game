//! The one place for balance numbers (AGENTS.md: no magic numbers in systems). Values are
//! placeholders until play tests and AI self-play settle them (ki-und-rl.md).

use bevy::ecs::prelude::Resource;

use crate::attributes::{Attribute, AttributeValues};

#[derive(Resource, Clone, Debug)]
pub struct Balance {
    /// Base attributes of a unit without blueprint, in thousandths.
    pub unit_base: AttributeValues,
    /// Lower and upper bound of each effective attribute, in thousandths.
    bounds: [(i64, i64); Attribute::COUNT],
    /// How many modifiers of one effect on one attribute a unit can carry from different sources.
    pub max_stacks: u8,
}

impl Balance {
    pub fn bounds(&self, attribute: Attribute) -> (i64, i64) {
        self.bounds[attribute as usize]
    }
}

impl Default for Balance {
    fn default() -> Self {
        let mut unit_base = AttributeValues::default();
        let mut bounds = [(0, 0); Attribute::COUNT];
        for (attribute, base, min, max) in [
            (Attribute::MaxHealth, 100_000, 1_000, 10_000_000),
            (Attribute::MaxMana, 100_000, 0, 10_000_000),
            (Attribute::ManaRegen, 5_000, 0, 1_000_000),
            (Attribute::MoveSpeed, 5_000, 0, 20_000),
            (Attribute::SpellPower, 1_000, 0, 10_000),
            (Attribute::Resistance, 1_000, 0, 10_000),
            (Attribute::CastSpeed, 1_000, 100, 10_000),
            (Attribute::Stealth, 0, 0, 10_000),
            (Attribute::Perception, 1_000, 0, 10_000),
        ] {
            unit_base[attribute] = base;
            bounds[attribute as usize] = (min, max);
        }
        Self {
            unit_base,
            bounds,
            max_stacks: 3,
        }
    }
}
