use crate::enemies::Enemy;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Projectile {
    pub acceleration: f32,
    pub agility: f32,
    pub direction: Vec3,
    pub homing: bool,
    pub activation_timer: f32,
    pub enemy: Option<Entity>,
    pub mesh_rotation_offset: Quat,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            acceleration: 0.0,
            agility: 0.0,
            direction: Vec3::Z,
            homing: false,
            activation_timer: 0.0,
            enemy: None,
            mesh_rotation_offset: Quat::IDENTITY,
        }
    }
}

pub fn despawn_out_of_bounds_projectiles(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
) {
    let boundary = 50.0; // Distance from origin before despawning

    for (entity, transform) in projectiles.iter() {
        let pos = transform.translation;
        // Despawn if projectile is too far in any direction
        if pos.x.abs() > boundary || pos.y.abs() > boundary || pos.z.abs() > boundary {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_projectile_activation_timers(
    mut projectiles: Query<&mut Projectile>,
    time: Res<Time>,
) {
    for mut projectile in projectiles.iter_mut() {
        if projectile.activation_timer > 0.0 {
            projectile.activation_timer -= time.delta_secs();
            if projectile.activation_timer < 0.0 {
                projectile.activation_timer = 0.0;
            }
        }
    }
}

pub fn select_projectile_enemies(
    mut projectiles: Query<(Entity, &mut Projectile, &Transform)>,
    enemies: Query<Entity, (With<Enemy>, With<Transform>)>,
) {
    let mut rng = rand::thread_rng();
    let enemy_entities: Vec<Entity> = enemies.iter().collect();

    if enemy_entities.is_empty() {
        return;
    }

    for (_projectile_entity, mut projectile, _transform) in projectiles.iter_mut() {
        if projectile.homing && projectile.activation_timer <= 0.0 && projectile.enemy.is_none() {
            // Randomly select an enemy
            let random_index = rng.gen_range(0..enemy_entities.len());
            projectile.enemy = Some(enemy_entities[random_index]);
        }
    }
}

pub fn apply_projectile_acceleration(
    mut projectiles: Query<(&Projectile, &mut Velocity)>,
    time: Res<Time>,
) {
    for (projectile, mut velocity) in projectiles.iter_mut() {
        if projectile.homing && projectile.activation_timer <= 0.0 {
            // Apply acceleration in the direction the projectile is pointing
            let acceleration_vector = projectile.direction * projectile.acceleration;
            velocity.linvel += acceleration_vector * time.delta_secs();
        }
    }
}

pub fn steer_projectiles_toward_enemy(
    mut projectiles: Query<(&mut Projectile, &mut Transform)>,
    enemies: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
    time: Res<Time>,
) {
    for (mut projectile, mut transform) in projectiles.iter_mut() {
        if !projectile.homing {
            continue;
        }

        if let Some(enemy_entity) = projectile.enemy {
            // Check if enemy still exists
            if let Ok(enemy_transform) = enemies.get(enemy_entity) {
                let projectile_pos = transform.translation;
                let enemy_pos = enemy_transform.translation;

                // Calculate desired direction to enemy
                let desired_direction = (enemy_pos - projectile_pos).normalize();

                // Rotate current direction toward desired direction using agility
                let current_direction = projectile.direction;
                let angle_between = current_direction.dot(desired_direction).acos();

                if angle_between > 0.001 {
                    // Calculate maximum rotation this frame
                    let max_rotation = projectile.agility * time.delta_secs();
                    let rotation_amount = angle_between.min(max_rotation);

                    // Use slerp to rotate toward enemy
                    let t = rotation_amount / angle_between;
                    projectile.direction = current_direction.lerp(desired_direction, t).normalize();

                    // Update transform rotation to match direction
                    // Use looking_to to create rotation that points in the direction vector
                    if projectile.direction.length_squared() > 0.001 {
                        // Create a temporary transform to calculate the base rotation
                        let mut temp_transform = Transform::IDENTITY;
                        temp_transform.look_to(projectile.direction, Vec3::Y);
                        // Apply the mesh rotation offset to preserve the initial mesh orientation
                        transform.rotation =
                            temp_transform.rotation * projectile.mesh_rotation_offset;
                    }
                } else {
                    // Already pointing at enemy, just update direction
                    projectile.direction = desired_direction;
                    // Update rotation to match
                    let mut temp_transform = Transform::IDENTITY;
                    temp_transform.look_to(desired_direction, Vec3::Y);
                    // Apply the mesh rotation offset to preserve the initial mesh orientation
                    transform.rotation = temp_transform.rotation * projectile.mesh_rotation_offset;
                }
            } else {
                // Enemy was despawned, clear enemy
                projectile.enemy = None;
            }
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_projectile_activation_timers,
                select_projectile_enemies,
                apply_projectile_acceleration,
                steer_projectiles_toward_enemy,
                despawn_out_of_bounds_projectiles,
            )
                .chain(),
        );
    }
}
