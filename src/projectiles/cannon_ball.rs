use bevy::prelude::*;
use crate::projectile::Projectile;
use crate::movable::Movable;

/// Spawns a cannon ball projectile
pub fn spawn_cannon_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    velocity: Vec3,
) {
    let projectile_mesh = meshes.add(Sphere::new(0.03));
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan projectile
        emissive: Color::srgb(1.0, 1.0, 0.0).into(),
        ..default()
    });
    
    commands.spawn((
        Projectile::default(),
        Movable::with_velocity(velocity, 1.0),
        Mesh3d(projectile_mesh),
        MeshMaterial3d(projectile_material),
        Transform::from_translation(position),
    ));
}
