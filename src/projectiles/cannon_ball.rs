use bevy::prelude::*;

/// Creates and returns the mesh and material handles for a cannon ball projectile
pub fn create_cannon_ball_mesh_and_material(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (Handle<Mesh>, Handle<StandardMaterial>) {
    let projectile_mesh = meshes.add(Sphere::new(0.03));
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan projectile
        emissive: Color::srgb(1.0, 1.0, 0.0).into(),
        ..default()
    });

    (projectile_mesh, projectile_material)
}
