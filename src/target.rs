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
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Spawn 5 targets at different positions (all at z=0)
    let positions = [
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(3.0, 2.0, 0.0),
        Vec3::new(3.0, -2.0, 0.0),
        Vec3::new(3.0, 1.0, 0.0),
        Vec3::new(3.0, -1.0, 0.0),
    ];

    for position in positions.iter() {
        spawn_target(&mut commands, &asset_server, &mut scene_spawner, *position);
    }
}

pub fn spawn_target(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    position: Vec3,
) {
    // Load the drone model
    let drone_handle = asset_server.load("models/enemies/drone.glb#Scene0");

    // Targets move slowly to the left (negative X direction)
    let left_velocity = Vec3::new(-0.2, 0.0, 0.0);

    let target_entity = commands
        .spawn((
            Target::default(),
            Collidable::new(0.25, 20.0, TARGET_HIT_POINTS, Team::Enemy), // 0.25 radius, 20 damage, 20 HP, enemy team
            Movable::with_velocity(left_velocity, 1.0), // No damping, constant velocity
            Transform {
                translation: position,
                rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0), // Rotate 90 degrees left to face movement direction
                scale: Vec3::splat(0.1),
            },
        ))
        .id();

    scene_spawner.spawn_as_child(drone_handle, target_entity);
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
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    targets: Query<(Entity, &Target, &Collidable, &Transform), (With<Target>, Without<Projectile>)>,
    mut player_score: ResMut<PlayerScore>,
) {
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
                &asset_server,
                &mut scene_spawner,
                new_position,
            );

            // Despawn the dead target
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_out_of_bounds_targets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    targets: Query<(Entity, &Transform), (With<Target>, Without<Projectile>)>,
) {
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
                &asset_server,
                &mut scene_spawner,
                new_position,
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
