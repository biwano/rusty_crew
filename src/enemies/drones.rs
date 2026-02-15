use crate::collision::{Collidable, Team};
use crate::weapons::cannon::create_cannon;
use crate::weapons::weapon::{Weapon, attach_weapon, fire_weapon};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn drone_behave(
    entity: Entity,
    weapon: &mut Weapon,
    transforms: &Query<&Transform>,
    velocities: &Query<&Velocity>,
    collidables: &Query<&Collidable>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
) {
    fire_weapon(
        weapon,
        entity,
        transforms,
        velocities,
        collidables,
        commands,
        meshes,
        materials,
        asset_server,
        scene_spawner,
    );
}

pub fn spawn_drone(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
) {
    // Load the drone model
    let drone_handle = asset_server.load("models/enemies/drone.glb#Scene0");

    // Enemies move slowly to the left (negative X direction)
    let left_velocity = Vec3::new(-0.2, 0.0, 0.0);

    let drone_entity = commands
        .spawn((
            super::Enemy {
                score: 100,
                behave: Some(drone_behave),
            },
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

    // Attach cannon to the drone, rotated 90 degrees around Y axis
    let mut cannon = create_cannon(Vec3::ZERO);
    cannon.fire_cooldown_duration *= 50.0; // Drones fire 10x slower than the default cannon
    cannon.projectile_spawn_speed_vector *= 0.1; // Projectiles are 10x slower
    attach_weapon(
        commands,
        asset_server,
        scene_spawner,
        drone_entity,
        cannon,
        Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
        Vec3::splat(1.0),
    );
}
