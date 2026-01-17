use crate::movable::Movable;
use crate::projectiles::Projectile;
use crate::projectiles::create_cannon_ball_mesh_and_material;
use crate::weapons::weapon::Weapon;
use bevy::prelude::*;

/// Spawns a cannon ball projectile using mesh information from cannon_ball module
pub fn spawn_cannon_ball_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    velocity: Vec3,
) {
    let (projectile_mesh, projectile_material) =
        create_cannon_ball_mesh_and_material(meshes, materials);

    commands.spawn((
        Projectile::default(),
        Movable::with_velocity(velocity, 1.0),
        Mesh3d(projectile_mesh),
        MeshMaterial3d(projectile_material),
        Transform::from_translation(position),
    ));
}

/// Creates a cannon weapon with specified position offset
pub fn create_cannon(weapon_position: Vec3) -> Weapon {
    Weapon::new()
        .with_fire_cooldown(0.5)
        .with_projectile_spawner(spawn_cannon_ball_projectile)
        .with_mesh_spawner(spawn_cannon_mesh)
        .with_weapon_position_offset(weapon_position)
        .with_projectile_spawn_offset(Vec3::new(0.5, 0.0, 0.0)) // Spawn projectiles 0.5 units to the right (forward) of weapon
}

/// Spawns the cannon mesh as a child of the given parent entity
pub fn spawn_cannon_mesh(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    parent_entity: Entity,
    translation: Vec3,
) {
    let weapon_mesh_handle = asset_server.load("models/cannon.glb#Scene0");
    let weapon_mesh_entity = commands
        .spawn((Transform {
            translation, // Position relative to ship
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2), // Rotate 90 degrees around Z axis
            scale: Vec3::splat(10.0), // Scale to match ship scale
        },))
        .id();
    scene_spawner.spawn_as_child(weapon_mesh_handle, weapon_mesh_entity);
    // Attach weapon mesh as a child of the parent
    commands.entity(parent_entity).add_child(weapon_mesh_entity);
}
