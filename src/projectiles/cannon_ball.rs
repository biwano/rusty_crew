use crate::collision::{Collidable, Team};
use crate::projectiles::Projectile;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Spawns a cannon ball projectile using mesh information from this module
pub fn spawn_cannon_ball_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    _asset_server: &Res<AssetServer>,
    _scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
    velocity: Vec3,
    rotation: Quat,
    team: Team,
) {
    let projectile_mesh = meshes.add(Sphere::new(0.03));
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan projectile
        emissive: Color::srgb(1.0, 1.0, 0.0).into(),
        ..default()
    });

    // Calculate forward direction from rotation
    let forward_direction = rotation * Vec3::Z;

    commands.spawn((
        Projectile {
            acceleration: 0.0,
            agility: 0.0,
            direction: forward_direction.normalize(),
            homing: false,
            activation_timer: 0.0,
            enemy: None,
            mesh_rotation_offset: Quat::IDENTITY, // No mesh offset for cannon balls
        },
        Collidable::new(10.0, 1.0, team), // 10 damage, 1 HP, use passed team
        Velocity::linear(velocity),
        RigidBody::KinematicVelocityBased,
        Collider::ball(0.03),
        ActiveEvents::COLLISION_EVENTS,
        Mesh3d(projectile_mesh),
        MeshMaterial3d(projectile_material),
        Transform {
            translation: position,
            rotation,
            scale: Vec3::ONE,
        },
    ));
}
