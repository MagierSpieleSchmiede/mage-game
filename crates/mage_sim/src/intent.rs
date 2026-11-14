//! What a unit wants to do in a tick, and how it gets there: producers (human input, AI,
//! add-ons, tests) write into the [`InputBuffer`] between ticks; the set Intent freezes it into
//! each unit's [`IntentForTick`] (intent-steuerung.md, ADR-0002, ADR-0004, ADR-0031).

use std::collections::BTreeMap;

use bevy::ecs::prelude::{Component, Query, ResMut, Resource};
use bevy::math::{Vec2, Vec3};

use crate::ids::{BehaviorId, PlayerId, SimId};

/// Slots on an ability bar, and so buttons in an intent (ADR-0030).
pub const MAX_SLOTS: usize = 12;

/// Per-tick control data of one unit. No `Entity`, no camera: this is what will go over the
/// network.
#[derive(Clone, Debug, PartialEq)]
pub struct MageIntent {
    /// Movement in the ground plane, already turned into world space; length at most 1.
    pub move_axis: Vec2,
    /// Aim in world space, normalised.
    pub aim_dir: Vec3,
    /// Slot held down in this tick.
    pub held: [bool; MAX_SLOTS],
    /// Slot pressed at least once since the previous tick.
    pub pressed: [bool; MAX_SLOTS],
}

impl Default for MageIntent {
    fn default() -> Self {
        Self {
            move_axis: Vec2::ZERO,
            aim_dir: Vec3::NEG_Z,
            held: [false; MAX_SLOTS],
            pressed: [false; MAX_SLOTS],
        }
    }
}

/// The intent a unit acts on in the current tick, plus the edge derived from the tick before.
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct IntentForTick {
    pub intent: MageIntent,
    /// Held in the previous tick, not in this one.
    pub just_released: [bool; MAX_SLOTS],
}

/// Who steers a unit right now (ADR-0031). Every steered unit gets an [`IntentForTick`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[require(IntentForTick)]
pub enum Controller {
    Player(PlayerId),
    Ai(BehaviorId),
}

/// Input of one unit collected since the last tick. Axes and held buttons are samples (the
/// last write wins); presses are counted so that none is lost (ADR-0004).
#[derive(Clone, Debug, Default)]
pub struct BufferedInput {
    pub move_axis: Vec2,
    /// Zero or not finite means "no new aim": the unit keeps its previous one.
    pub aim_dir: Vec3,
    held: [bool; MAX_SLOTS],
    press_count: [u8; MAX_SLOTS],
}

impl BufferedInput {
    /// Records a press of `slot`. Slots beyond the bar are ignored.
    pub fn press(&mut self, slot: usize) {
        if let Some(count) = self.press_count.get_mut(slot) {
            *count = count.saturating_add(1);
        }
    }

    /// Sets whether `slot` is held. Slots beyond the bar are ignored.
    pub fn set_held(&mut self, slot: usize, held: bool) {
        if let Some(state) = self.held.get_mut(slot) {
            *state = held;
        }
    }
}

/// Input for every steered unit, by its `SimId`. Written outside the simulation, consumed by
/// the set Intent. Entries for units that do not exist are never read.
#[derive(Resource, Debug, Default)]
pub struct InputBuffer(BTreeMap<SimId, BufferedInput>);

impl InputBuffer {
    /// The input of `unit`, created empty if there is none yet.
    pub fn entry(&mut self, unit: SimId) -> &mut BufferedInput {
        self.0.entry(unit).or_default()
    }
}

/// Set Intent: freezes each unit's buffered input into its [`IntentForTick`], derives
/// `pressed` from the press counters and resets them, derives `just_released` from the
/// previous tick, and drops values no honest producer sends (sicherheit.md).
pub fn freeze_intents(
    mut buffer: ResMut<InputBuffer>,
    mut units: Query<(&SimId, &mut IntentForTick)>,
) {
    for (sim_id, mut frozen) in &mut units {
        let previous_held = frozen.intent.held;
        let mut intent = MageIntent {
            aim_dir: frozen.intent.aim_dir,
            ..MageIntent::default()
        };
        if let Some(input) = buffer.0.get_mut(sim_id) {
            intent.move_axis = if input.move_axis.is_finite() {
                input.move_axis.clamp_length_max(1.0)
            } else {
                Vec2::ZERO
            };
            if let Some(aim) = input.aim_dir.try_normalize() {
                intent.aim_dir = aim;
            }
            intent.held = input.held;
            for (pressed, count) in intent.pressed.iter_mut().zip(&mut input.press_count) {
                *pressed = *count > 0;
                *count = 0;
            }
        }
        let mut just_released = [false; MAX_SLOTS];
        for (slot, released) in just_released.iter_mut().enumerate() {
            *released = previous_held[slot] && !intent.held[slot];
        }
        *frozen = IntentForTick {
            intent,
            just_released,
        };
    }
}
