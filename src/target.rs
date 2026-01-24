use crate::collision::{Collidable, Team};
use crate::hud::PlayerScore;
use crate::movable::Movable;
use crate::projectiles::Projectile;
use bevy::prelude::*;

const TARGET_HIT_POINTS: f32 = 20.0;

#[derive(Component)]
pub struct Target {
    pub score: u32,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            score: 100, // Default score for destroying a target
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
        spawn_target(&mut commands, &mut materials, *position, cube_mesh.clone());
    }
}

pub fn spawn_target(
    commands: &mut Commands,
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
        Collidable::new(0.25, 20.0, TARGET_HIT_POINTS, Team::Enemy), // 0.25 radius, 20 damage, 20 HP, enemy team
        Movable::with_velocity(left_velocity, 1.0), // No damping, constant velocity
        Mesh3d(cube_mesh),
        MeshMaterial3d(target_material),
        Transform::from_translation(position),
    ));
}

pub fn update_target_colors(
    targets: Query<(&Collidable, &MeshMaterial3d<StandardMaterial>), With<Target>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (collidable, material_handle) in targets.iter() {
        // Calculate color based on hit points
        // Green when healthy (max HP), red when dead (0 HP)
        let health_ratio = (collidable.hit_points / collidable.max_hit_points).clamp(0.0, 1.0);
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
    targets: Query<(Entity, &Target, &Collidable, &Transform), (With<Target>, Without<Projectile>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_score: ResMut<PlayerScore>,
) {
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    for (entity, target, collidable, _transform) in targets.iter() {
        if collidable.hit_points <= 0.0 {
            // Add score to player
            player_score.score += target.score;

            // Spawn a new target at a random position on the right side
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let new_y = rng.gen_range(-2.0..2.0);
            let new_position = Vec3::new(3.0, new_y, 0.0);

            spawn_target(
                &mut commands,
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
                &mut materials,
                new_position,
                cube_mesh.clone(),
            );

            // Despawn the out-of-bounds target
            commands.entity(entity).despawn();
        }
    }
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_targets).add_systems(
            Update,
            (
                update_target_colors,
                despawn_dead_targets,
                despawn_out_of_bounds_targets,
            ),
        );
    }
}
