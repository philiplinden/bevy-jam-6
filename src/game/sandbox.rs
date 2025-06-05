//! This module is the box that particles are spawned in.

use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ScreenWrap>();
    app.insert_resource(PhysicsBoundary::default());
    app.add_systems(
        Update,
        (
            apply_screen_wrap,
            apply_physics_boundary.run_if(resource_changed::<PhysicsBoundary>),
        ),
    );
}

/// A marker component that indicates an entity should be wrapped around the screen
/// when it moves outside the boundaries of the physics simulation area.
#[derive(Component, Reflect, Debug, Clone, PartialEq)]
#[reflect(Component)]
pub struct ScreenWrap;

/// This resource defines the boundaries of the physics simulation area. It
/// allows for the wraparound effects to be toggled on and off for each side of
/// the simulation area.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct PhysicsBoundary {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl Default for PhysicsBoundary {
    fn default() -> Self {
        Self {
            top: true,
            bottom: true,
            left: false,
            right: false,
        }
    }
}

#[derive(Component)]
pub struct WallCollider;

fn apply_physics_boundary(
    mut commands: Commands,
    boundary: Res<PhysicsBoundary>,
    window: Single<&Window, With<PrimaryWindow>>,
    existing_walls: Query<Entity, With<WallCollider>>,
) {
    // Remove existing wall colliders
    for entity in &existing_walls {
        commands.entity(entity).despawn();
    }

    let size = window.size();
    let half_width = size.x;
    let half_height = size.y / 4.0;

    // Top wall
    if boundary.top {
        commands.spawn((
            WallCollider,
            RigidBody::Static,
            Collider::segment(
                Vec2::new(-half_width, half_height),
                Vec2::new(half_width, half_height),
            ),
            Transform::default(),
        ));
    }
    // Bottom wall
    if boundary.bottom {
        commands.spawn((
            WallCollider,
            RigidBody::Static,
            Collider::segment(
                Vec2::new(-half_width, -half_height),
                Vec2::new(half_width, -half_height),
            ),
            Transform::default(),
        ));
    }
    // Left wall
    if boundary.left {
        commands.spawn((
            WallCollider,
            RigidBody::Static,
            Collider::segment(
                Vec2::new(-half_width, -half_height),
                Vec2::new(-half_width, half_height),
            ),
            Transform::default(),
        ));
    }
    // Right wall
    if boundary.right {
        commands.spawn((
            WallCollider,
            RigidBody::Static,
            Collider::segment(
                Vec2::new(half_width, -half_height),
                Vec2::new(half_width, half_height),
            ),
            Transform::default(),
        ));
    }
}

fn apply_screen_wrap(
    window: Single<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window.size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let mut position = transform.translation.xy();
        position.x = (position.x + half_size.x).rem_euclid(size.x) - half_size.x;
        position.y = (position.y + half_size.y).rem_euclid(size.y) - half_size.y;
        transform.translation = position.extend(transform.translation.z);
    }
}
