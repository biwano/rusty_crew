use crate::projectiles::spawn_cannon_ball_projectile;
use crate::weapons::weapon::{Weapon, WeaponMesh};
use bevy::prelude::*;

/// Creates a cannon weapon with specified position offset
pub fn create_cannon(weapon_position: Vec3) -> Weapon {
    Weapon::new()
        .with_fire_cooldown(0.1)
        .with_projectile_spawner(spawn_cannon_ball_projectile)
        .with_mesh_spawner(spawn_cannon_mesh)
        .with_weapon_position_offset(weapon_position)
        .with_projectile_spawn_offset(Vec3::new(0.15, 0.05, 0.0)) // Initial position of the projectile relative to the weapon
        .with_projectile_spawn_speed_vector(Vec3::new(15.0, 0.0, 0.0)) // Cannon balls are fast
}

/// Spawns the cannon mesh as a child of the given parent entity
pub fn spawn_cannon_mesh(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    parent_entity: Entity,
    translation: Vec3,
) {
    let weapon_mesh_handle = asset_server.load("models/weapons/cannon.glb#Scene0");
    let weapon_mesh_entity = commands
        .spawn((
            Transform {
                translation, // Position relative to ship
                rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2), // Rotate 90 degrees around Z axis
                scale: Vec3::splat(10.0), // Scale to match ship scale
            },
            WeaponMesh,
        ))
        .id();
    scene_spawner.spawn_as_child(weapon_mesh_handle, weapon_mesh_entity);
    // Attach weapon mesh as a child of the parent
    commands.entity(parent_entity).add_child(weapon_mesh_entity);
}
