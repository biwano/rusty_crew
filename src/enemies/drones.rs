use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::collision::{Collidable, Team};

pub fn spawn_drone(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    _scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
) {
    // Load the drone model
    let drone_handle = asset_server.load("models/enemies/drone.glb#Scene0");

    // Enemies move slowly to the left (negative X direction)
    let left_velocity = Vec3::new(-0.2, 0.0, 0.0);

    let _enemy_entity = commands
        .spawn((
            super::Enemy::default(),
            Collidable::new(20.0, super::ENEMY_HIT_POINTS, Team::Enemy), // 20 damage, 20 HP, enemy team
            Velocity::linear(left_velocity),
            RigidBody::KinematicVelocityBased,
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            Transform {
                translation: position,
                rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0), // Rotate 90 degrees left to face movement direction
                scale: Vec3::splat(0.1),
            },
            SceneRoot(drone_handle),
            AsyncSceneCollider {
                shape: Some(ComputedColliderShape::ConvexHull),
                named_shapes: Default::default(),
            },
        ))
        .id();
}
