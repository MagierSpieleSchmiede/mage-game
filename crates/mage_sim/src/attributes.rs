//! How strong and able a unit is right now: base values plus modifiers, recomputed every tick
//! in the set Werte (attribute.md, ADR-0016). All values are integers in thousandths, so the
//! result is the same on every machine.

use std::ops::{Index, IndexMut};

use bevy::ecs::prelude::{Component, Query, Res};

use crate::balance::Balance;
use crate::ids::{EffectId, SimId};

/// The attributes of a unit. A new one is a variant here, a base value in the balance and a
/// place that reads its effective value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    MaxHealth,
    MaxMana,
    /// Mana per second.
    ManaRegen,
    /// Metres per second.
    MoveSpeed,
    /// Factor on damage dealt.
    SpellPower,
    /// Factor on damage taken.
    Resistance,
    /// Factor on cooldowns and cast times.
    CastSpeed,
    Stealth,
    Perception,
}

impl Attribute {
    pub const COUNT: usize = 9;
    pub const ALL: [Attribute; Self::COUNT] = [
        Attribute::MaxHealth,
        Attribute::MaxMana,
        Attribute::ManaRegen,
        Attribute::MoveSpeed,
        Attribute::SpellPower,
        Attribute::Resistance,
        Attribute::CastSpeed,
        Attribute::Stealth,
        Attribute::Perception,
    ];
}

/// One value per attribute, in thousandths.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AttributeValues(pub [i64; Attribute::COUNT]);

impl Index<Attribute> for AttributeValues {
    type Output = i64;
    fn index(&self, attribute: Attribute) -> &i64 {
        &self.0[attribute as usize]
    }
}

impl IndexMut<Attribute> for AttributeValues {
    fn index_mut(&mut self, attribute: Attribute) -> &mut i64 {
        &mut self.0[attribute as usize]
    }
}

/// Values before any modifier: from the balance, later from a creature's blueprint.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BaseAttributes(pub AttributeValues);

/// Values after all modifiers. The only attribute values other systems read (invariant 12).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectiveAttributes(pub AttributeValues);

impl EffectiveAttributes {
    pub fn get(&self, attribute: Attribute) -> i64 {
        self.0[attribute]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModifierKind {
    /// Adds `value` thousandths.
    Add,
    /// Multiplies by 1 + `value` / 1000; -500 halves, 250 adds a quarter.
    Mul,
}

/// What a modifier does when its effect hits a unit that already has it from the same source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stacking {
    /// Replace it, which renews the duration (the default for effects).
    Refresh,
    /// Add another one.
    Stack,
}

/// A temporary or permanent change of one attribute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Modifier {
    pub effect: EffectId,
    pub attribute: Attribute,
    pub kind: ModifierKind,
    pub value: i64,
    /// Ticks it still acts; `None` lasts until removed (equipment, later).
    pub remaining_ticks: Option<u32>,
    /// The unit or spell it came from; decides the order of calculation.
    pub source: SimId,
}

/// The modifiers on a unit.
#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub struct Modifiers(Vec<Modifier>);

impl Modifiers {
    /// Adds `modifier` by the stacking rule. Several sources stack up to `max_stacks` of the
    /// same effect on the same attribute; beyond that a new one is ignored.
    pub fn add(&mut self, modifier: Modifier, stacking: Stacking, max_stacks: u8) {
        let same_effect =
            |m: &Modifier| m.effect == modifier.effect && m.attribute == modifier.attribute;
        if stacking == Stacking::Refresh
            && let Some(existing) = self
                .0
                .iter_mut()
                .find(|m| same_effect(m) && m.source == modifier.source)
        {
            *existing = modifier;
            return;
        }
        if self.0.iter().filter(|m| same_effect(m)).count() < usize::from(max_stacks) {
            self.0.push(modifier);
        }
    }

    pub fn list(&self) -> &[Modifier] {
        &self.0
    }
}

/// The effective value of `attribute`: (base + all additions) × each factor, factors in the
/// order of their source's `SimId`, every step truncated to whole thousandths, clamped to
/// `bounds`. A factor never drops below zero.
pub fn effective_value(
    attribute: Attribute,
    base: i64,
    modifiers: &[Modifier],
    (min, max): (i64, i64),
) -> i64 {
    let mut relevant: Vec<&Modifier> = modifiers
        .iter()
        .filter(|m| m.attribute == attribute)
        .collect();
    relevant.sort_by_key(|m| m.source);
    let mut value = relevant
        .iter()
        .filter(|m| m.kind == ModifierKind::Add)
        .fold(i128::from(base), |sum, m| sum + i128::from(m.value));
    for m in relevant.iter().filter(|m| m.kind == ModifierKind::Mul) {
        let factor = (1000 + i128::from(m.value)).max(0);
        value = value * factor / 1000;
    }
    value.clamp(i128::from(min), i128::from(max)) as i64
}

/// Set Werte: drops expired modifiers, recomputes [`EffectiveAttributes`], then counts the
/// remaining durations down. A modifier added with N ticks acts in exactly the next N ticks.
pub fn update_attributes(
    balance: Res<Balance>,
    mut units: Query<(&BaseAttributes, &mut Modifiers, &mut EffectiveAttributes)>,
) {
    for (base, mut modifiers, mut effective) in &mut units {
        modifiers.0.retain(|m| m.remaining_ticks != Some(0));
        for attribute in Attribute::ALL {
            effective.0[attribute] = effective_value(
                attribute,
                base.0[attribute],
                &modifiers.0,
                balance.bounds(attribute),
            );
        }
        for m in &mut modifiers.0 {
            if let Some(remaining) = &mut m.remaining_ticks {
                *remaining = remaining.saturating_sub(1);
            }
        }
    }
}
