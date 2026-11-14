//! B-007: attributes are base values plus modifiers, recomputed in the set Werte in integer
//! thousandths, in source order, clamped, with durations in ticks (attribute.md, ADR-0016).

use bevy::app::App;
use bevy::ecs::component::{Component, Mutable};
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::{Mut, With};
use mage_sim::attributes::{
    Attribute, BaseAttributes, EffectiveAttributes, Modifier, ModifierKind, Modifiers, Stacking,
    effective_value,
};
use mage_sim::balance::Balance;
use mage_sim::headless::{headless_app, step_ticks};
use mage_sim::ids::{EffectId, SimId};
use mage_sim::mage::Mage;
use mage_sim::scenario::Scenario;

const SLOW: EffectId = EffectId(1);
const HASTE: EffectId = EffectId(2);

fn modifier(
    attribute: Attribute,
    kind: ModifierKind,
    value: i64,
    source: u64,
    remaining: Option<u32>,
) -> Modifier {
    Modifier {
        effect: SLOW,
        attribute,
        kind,
        value,
        remaining_ticks: remaining,
        source: SimId(source),
    }
}

#[test]
fn additive_then_multiplicative_in_thousandths() {
    // (100 + 20) × 1.5 × 0.8 = 144
    let modifiers = [
        modifier(Attribute::MaxHealth, ModifierKind::Add, 20_000, 1, None),
        modifier(Attribute::MaxHealth, ModifierKind::Mul, 500, 2, None),
        modifier(Attribute::MaxHealth, ModifierKind::Mul, -200, 3, None),
    ];
    assert_eq!(
        effective_value(Attribute::MaxHealth, 100_000, &modifiers, (0, i64::MAX)),
        144_000
    );
}

#[test]
fn integer_steps_are_fixed_and_follow_source_order() {
    // Source order: 1.001 × 1.333 = 1.334 (truncated), × 0.75 = 1.000 (truncated).
    // The other order would give 0.999, so the order of sources must decide, not the list.
    let up = modifier(Attribute::SpellPower, ModifierKind::Mul, 333, 1, None);
    let down = modifier(Attribute::SpellPower, ModifierKind::Mul, -250, 2, None);
    let in_order = effective_value(
        Attribute::SpellPower,
        1_001,
        &[up.clone(), down.clone()],
        (0, i64::MAX),
    );
    let reversed = effective_value(Attribute::SpellPower, 1_001, &[down, up], (0, i64::MAX));
    assert_eq!(in_order, 1_000);
    assert_eq!(reversed, 1_000);
}

#[test]
fn results_are_clamped_and_a_factor_never_goes_negative() {
    let crush = modifier(Attribute::MoveSpeed, ModifierKind::Mul, -3_000, 1, None);
    assert_eq!(
        effective_value(Attribute::MoveSpeed, 5_000, &[crush], (0, 20_000)),
        0
    );
    let boost = modifier(Attribute::MoveSpeed, ModifierKind::Add, 100_000, 1, None);
    assert_eq!(
        effective_value(Attribute::MoveSpeed, 5_000, &[boost], (0, 20_000)),
        20_000
    );
}

#[test]
fn modifiers_of_other_attributes_do_not_count() {
    let haste = modifier(Attribute::CastSpeed, ModifierKind::Mul, 300, 1, None);
    assert_eq!(
        effective_value(Attribute::MoveSpeed, 5_000, &[haste], (0, 20_000)),
        5_000
    );
}

#[test]
fn the_same_source_refreshes_instead_of_stacking() {
    let mut modifiers = Modifiers::default();
    let slow = modifier(Attribute::MoveSpeed, ModifierKind::Mul, -500, 7, Some(90));
    modifiers.add(slow.clone(), Stacking::Refresh, 3);
    let mut again = slow;
    again.remaining_ticks = Some(90);
    modifiers.add(again, Stacking::Refresh, 3);
    assert_eq!(modifiers.list().len(), 1);
}

#[test]
fn different_sources_stack_up_to_the_cap() {
    let mut modifiers = Modifiers::default();
    for source in 0..5 {
        modifiers.add(
            modifier(
                Attribute::MoveSpeed,
                ModifierKind::Mul,
                -100,
                source,
                Some(90),
            ),
            Stacking::Refresh,
            3,
        );
    }
    assert_eq!(modifiers.list().len(), 3);
}

#[test]
fn a_stacking_effect_stacks_from_the_same_source() {
    let mut modifiers = Modifiers::default();
    for _ in 0..2 {
        let mut haste = modifier(Attribute::CastSpeed, ModifierKind::Mul, 100, 4, Some(30));
        haste.effect = HASTE;
        modifiers.add(haste, Stacking::Stack, 3);
    }
    assert_eq!(modifiers.list().len(), 2);
}

fn app() -> App {
    headless_app(Scenario::builder().team(1).team(1).build().unwrap())
}

fn first_mage<T>(app: &mut App) -> Mut<'_, T>
where
    T: Component<Mutability = Mutable>,
{
    let world = app.world_mut();
    let entity = world
        .query_filtered::<(Entity, &SimId), With<Mage>>()
        .iter(world)
        .find(|(_, sim_id)| **sim_id == SimId(0))
        .map(|(entity, _)| entity)
        .unwrap();
    world.get_mut::<T>(entity).unwrap()
}

fn speed(app: &mut App) -> i64 {
    first_mage::<EffectiveAttributes>(app).get(Attribute::MoveSpeed)
}

#[test]
fn units_start_with_the_base_values_of_the_balance() {
    let mut app = app();
    step_ticks(&mut app, 1);
    let balance_speed = app.world().resource::<Balance>().unit_base[Attribute::MoveSpeed];
    assert_eq!(
        first_mage::<BaseAttributes>(&mut app).0[Attribute::MoveSpeed],
        balance_speed
    );
    assert_eq!(speed(&mut app), balance_speed);
}

#[test]
fn a_modifier_for_n_ticks_acts_in_exactly_the_next_n_ticks() {
    let mut app = app();
    step_ticks(&mut app, 1);
    let base = speed(&mut app);
    // As if added in the set Schaden of the tick just run.
    first_mage::<Modifiers>(&mut app).add(
        modifier(Attribute::MoveSpeed, ModifierKind::Mul, -500, 1, Some(3)),
        Stacking::Refresh,
        3,
    );
    let mut seen = Vec::new();
    for _ in 0..5 {
        step_ticks(&mut app, 1);
        seen.push(speed(&mut app));
    }
    assert_eq!(seen, [base / 2, base / 2, base / 2, base, base]);
    assert!(first_mage::<Modifiers>(&mut app).list().is_empty());
}

#[test]
fn a_permanent_modifier_stays() {
    let mut app = app();
    first_mage::<Modifiers>(&mut app).add(
        modifier(Attribute::MoveSpeed, ModifierKind::Add, 1_000, 1, None),
        Stacking::Refresh,
        3,
    );
    step_ticks(&mut app, 100);
    let base = app.world().resource::<Balance>().unit_base[Attribute::MoveSpeed];
    assert_eq!(speed(&mut app), base + 1_000);
}
