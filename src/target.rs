use crate::movable::Movable;
use crate::projectiles::Projectile;
use bevy::prelude::*;

const TARGET_HIT_POINTS: f32 = 20.0;

#[derive(Component)]
pub struct Target {
    pub hit_points: f32,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            hit_points: TARGET_HIT_POINTS,
        }
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
        spawn_target(
            &mut commands,
            &mut meshes,
            &mut materials,
            *position,
            cube_mesh.clone(),
        );
    }
}

pub fn spawn_target(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    cube_mesh: Handle<Mesh>,
) {
    // Create a unique material for each target (green when healthy)
    let target_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.0), // Green when healthy
        ..default()
    });

    // Targets move slowly to the left (negative X direction)
    let left_velocity = Vec3::new(-0.2, 0.0, 0.0);

    commands.spawn((
        Target::default(),
        Movable::with_velocity(left_velocity, 1.0), // No damping, constant velocity
        Mesh3d(cube_mesh),
        MeshMaterial3d(target_material),
        Transform::from_translation(position),
    ));
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
        // Calculate color based on hit points
        // Green when healthy (max HP), red when dead (0 HP)
        let health_ratio = (target.hit_points / TARGET_HIT_POINTS).clamp(0.0, 1.0);
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
    targets: Query<(Entity, &Target, &Transform), (With<Target>, Without<Projectile>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    for (entity, target, _transform) in targets.iter() {
        if target.hit_points <= 0.0 {
            // Spawn a new target at a random position on the right side
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let new_y = rng.gen_range(-2.0..2.0);
            let new_position = Vec3::new(3.0, new_y, 0.0);

            spawn_target(
                &mut commands,
                &mut meshes,
                &mut materials,
                new_position,
                cube_mesh.clone(),
            );

            // Despawn the dead target
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_out_of_bounds_targets(
    mut commands: Commands,
    targets: Query<(Entity, &Transform), (With<Target>, Without<Projectile>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let left_boundary = -5.0; // Despawn targets that go too far to the left

    for (entity, transform) in targets.iter() {
        let pos = transform.translation;

        // If target has moved off-screen to the left, despawn and respawn
        if pos.x < left_boundary {
            // Spawn a new target at a random position on the right side
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let new_y = rng.gen_range(-2.0..2.0);
            let new_position = Vec3::new(3.0, new_y, 0.0);

            spawn_target(
                &mut commands,
                &mut meshes,
                &mut materials,
                new_position,
                cube_mesh.clone(),
            );

            // Despawn the out-of-bounds target
            commands.entity(entity).despawn();
        }
    }
}
