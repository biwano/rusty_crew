pub mod drones;

use crate::collision::Collidable;
use crate::hud::PlayerScore;
use crate::projectiles::Projectile;
use bevy::prelude::*;
use drones::spawn_drone;

pub const ENEMY_HIT_POINTS: f32 = 20.0;

#[derive(Component)]
pub struct Enemy {
    pub score: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            score: 100, // Default score for destroying an enemy
        }
    }
}

pub fn setup_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Spawn 5 enemies at different positions (all at z=0)
    let positions = [
        Vec3::new(3.0, 0.0, 0.0),
        Vec3::new(3.0, 2.0, 0.0),
        Vec3::new(3.0, -2.0, 0.0),
        Vec3::new(3.0, 1.0, 0.0),
        Vec3::new(3.0, -1.0, 0.0),
    ];

    for position in positions.iter() {
        spawn_drone(&mut commands, &asset_server, &mut scene_spawner, *position);
    }
}

pub fn update_enemy_colors(
    enemies: Query<(&Collidable, &MeshMaterial3d<StandardMaterial>), With<Enemy>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (collidable, material_handle) in enemies.iter() {
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

pub fn despawn_dead_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    enemies: Query<(Entity, &Enemy, &Collidable, &Transform), (With<Enemy>, Without<Projectile>)>,
    mut player_score: ResMut<PlayerScore>,
) {
    for (entity, enemy, collidable, _transform) in enemies.iter() {
        if collidable.hit_points <= 0.0 {
            // Add score to player
            player_score.score += enemy.score;

            // Spawn a new enemy at a random position on the right side
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let new_y = rng.gen_range(-2.0..2.0);
            let new_position = Vec3::new(3.0, new_y, 0.0);

            spawn_drone(
                &mut commands,
                &asset_server,
                &mut scene_spawner,
                new_position,
            );

            // Despawn the dead enemy
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_out_of_bounds_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    enemies: Query<(Entity, &Transform), (With<Enemy>, Without<Projectile>)>,
) {
    let left_boundary = -5.0; // Despawn enemies that go too far to the left

    for (entity, transform) in enemies.iter() {
        let pos = transform.translation;

        // If enemy has moved off-screen to the left, despawn and respawn
        if pos.x < left_boundary {
            // Spawn a new enemy at a random position on the right side
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let new_y = rng.gen_range(-2.0..2.0);
            let new_position = Vec3::new(3.0, new_y, 0.0);

            spawn_drone(
                &mut commands,
                &asset_server,
                &mut scene_spawner,
                new_position,
            );

            // Despawn the out-of-bounds enemy
            commands.entity(entity).despawn();
        }
    }
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies).add_systems(
            Update,
            (
                update_enemy_colors,
                despawn_dead_enemies,
                despawn_out_of_bounds_enemies,
            ),
        );
    }
}
