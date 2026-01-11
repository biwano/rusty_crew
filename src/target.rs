use bevy::prelude::*;
use crate::projectile::Projectile;

#[derive(Component)]
pub struct Target {
    pub hit_points: f32,
}

impl Default for Target {
    fn default() -> Self {
        Self { hit_points: 100.0 }
    }
}

pub fn setup_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a cube mesh for targets
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    
    // Spawn 5 cubes at different positions (all at z=0)
    let positions = [
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(3.0, 2.0, 0.0),
        Vec3::new(3.0, -2.0, 0.0),
        Vec3::new(3.0, 1.0, 0.0),
        Vec3::new(3.0, -1.0, 0.0),
    ];
    
    for position in positions.iter() {
        // Create a unique material for each target (green when healthy)
        let target_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0), // Green when healthy
            ..default()
        });
        
        commands.spawn((
            Target::default(),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(target_material),
            Transform::from_translation(*position),
        ));
    }
}

pub fn check_projectile_target_collisions(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Projectile)>,
    mut targets: Query<(Entity, &mut Target, &Transform)>,
) {
    let collision_distance = 0.5; // Collision detection distance
    
    for (projectile_entity, projectile_transform, projectile) in projectiles.iter() {
        let projectile_pos = projectile_transform.translation;
        
        for (_target_entity, mut target, target_transform) in targets.iter_mut() {
            let target_pos = target_transform.translation;
            let distance = projectile_pos.distance(target_pos);
            
            // If projectile is close enough to target, apply damage
            if distance < collision_distance {
                // Apply damage to target
                target.hit_points -= projectile.damage;
                
                // Despawn projectile
                commands.entity(projectile_entity).despawn();
                break; // Only hit one target per projectile
            }
        }
    }
}

pub fn update_target_colors(
    targets: Query<(&Target, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (target, material_handle) in targets.iter() {
        // Calculate color based on hit points (0-100)
        // Green when healthy (100 HP), red when dead (0 HP)
        let health_ratio = (target.hit_points / 100.0).clamp(0.0, 1.0);
        let red = 1.0 - health_ratio;
        let green = health_ratio;
        
        // Update the material color
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.base_color = Color::srgb(red, green, 0.0);
        }
    }
}

pub fn despawn_dead_targets(
    mut commands: Commands,
    targets: Query<(Entity, &Target)>,
) {
    for (entity, target) in targets.iter() {
        if target.hit_points <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

