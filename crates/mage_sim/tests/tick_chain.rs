//! B-005: the tick chain runs its sets in a fixed order, with Avian's physics step inside the
//! set Physik, so a shape cast in the set Treffer sees positions of the same tick
//! (simulationsmodell.md, P-005).

use avian3d::prelude::{
    Collider, PhysicsSystems, RigidBody, ShapeCastConfig, SpatialQuery, SpatialQueryFilter,
};
use bevy::app::{App, FixedUpdate};
use bevy::ecs::prelude::{Component, IntoScheduleConfigs, Query, ResMut, Resource, With};
use bevy::math::{Dir3, Quat, Vec3};
use bevy::transform::components::Transform;
use mage_sim::headless::{headless_app, step_ticks};
use mage_sim::scenario::Scenario;
use mage_sim::schedule::SimSet;

fn app() -> App {
    headless_app(Scenario::builder().team(1).team(1).build().unwrap())
}

#[derive(Resource, Default)]
struct Log(Vec<&'static str>);

fn add_probe(app: &mut App, name: &'static str, set: impl bevy::ecs::schedule::SystemSet) {
    app.add_systems(
        FixedUpdate,
        (move |mut log: ResMut<Log>| log.0.push(name)).in_set(set),
    );
}

#[test]
fn the_sets_run_in_chain_order_with_physics_between_movement_and_hits() {
    let mut app = app();
    app.init_resource::<Log>();
    add_probe(&mut app, "cleanup", SimSet::Cleanup);
    add_probe(&mut app, "visibility", SimSet::Visibility);
    add_probe(&mut app, "damage", SimSet::Damage);
    add_probe(&mut app, "hits", SimSet::Hits);
    add_probe(&mut app, "physics last", PhysicsSystems::Last);
    add_probe(&mut app, "physics first", PhysicsSystems::First);
    add_probe(&mut app, "movement", SimSet::Movement);
    add_probe(&mut app, "casting", SimSet::Casting);
    add_probe(&mut app, "values", SimSet::Values);
    add_probe(&mut app, "intent", SimSet::Intent);

    step_ticks(&mut app, 1);

    assert_eq!(
        app.world().resource::<Log>().0,
        [
            "intent",
            "values",
            "casting",
            "movement",
            "physics first",
            "physics last",
            "hits",
            "damage",
            "visibility",
            "cleanup",
        ]
    );
}

#[derive(Component)]
struct Target;

/// Distance of the shape-cast hit in the set Treffer, if any.
#[derive(Resource, Default)]
struct Seen(Option<f32>);

fn move_target_into_line_of_fire(mut targets: Query<&mut Transform, With<Target>>) {
    for mut transform in &mut targets {
        transform.translation = Vec3::new(10.0, 0.0, 0.0);
    }
}

fn shape_cast_along_x(spatial: SpatialQuery, mut seen: ResMut<Seen>) {
    let config = ShapeCastConfig {
        max_distance: 50.0,
        ..ShapeCastConfig::DEFAULT
    };
    seen.0 = spatial
        .cast_shape(
            &Collider::sphere(0.1),
            Vec3::ZERO,
            Quat::IDENTITY,
            Dir3::X,
            &config,
            &SpatialQueryFilter::default(),
        )
        .map(|hit| hit.distance);
}

#[test]
fn a_shape_cast_in_hits_sees_a_position_set_in_movement_of_the_same_tick() {
    let mut app = app();
    app.init_resource::<Seen>()
        .add_systems(
            FixedUpdate,
            move_target_into_line_of_fire.in_set(SimSet::Movement),
        )
        .add_systems(FixedUpdate, shape_cast_along_x.in_set(SimSet::Hits));
    // Far off the line of fire until the first tick moves it.
    app.world_mut().spawn((
        Target,
        RigidBody::Kinematic,
        Collider::sphere(0.5),
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));

    step_ticks(&mut app, 1);

    let distance = app
        .world()
        .resource::<Seen>()
        .0
        .expect("the cast hits the target in the tick it moved");
    // Centre at x = 10, target radius 0.5, cast radius 0.1.
    assert!((distance - 9.4).abs() < 1e-3, "distance {distance}");
}
