use crate::collision::{Collidable, Team};
use crate::projectiles::Projectile;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Spawns a rocket projectile using the rocket.glb mesh
pub fn spawn_rocket_projectile(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    _scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
    velocity: Vec3,
    rotation: Quat,
) {
    let rocket_scene_handle = asset_server.load("models/projectiles/rocket.glb#Scene0");

    // Calculate forward direction from rotation
    let rocket_rotation = rotation * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    let forward_direction = rocket_rotation * Vec3::Z;

    commands.spawn((
        Projectile {
            acceleration: 5.0, // Acceleration for rockets
            agility: 0.5,      // Turn rate in radians per second
            direction: forward_direction.normalize(),
            homing: true,          // Rockets are homing projectiles
            activation_timer: 1.0, // Start with 1 second cooldown
            target: None,          // No target initially
            mesh_rotation_offset: Quat::from_rotation_y(std::f32::consts::PI), // 90-degree Y rotation for rocket mesh
        },
        Collidable::new(25.0, 1.0, Team::Player), // 25 damage, 1 HP, player team
        Velocity::linear(velocity),
        Damping {
            linear_damping: 0.6,
            angular_damping: 0.0,
        },
        RigidBody::KinematicVelocityBased,
        SceneRoot(rocket_scene_handle),
        AsyncSceneCollider {
            shape: Some(ComputedColliderShape::ConvexHull),
            named_shapes: Default::default(),
        },
        ActiveEvents::COLLISION_EVENTS,
        Transform {
            translation: position,
            rotation: rocket_rotation,
            scale: Vec3::splat(0.0002), // Scale down the rocket
        },
    ));
}
