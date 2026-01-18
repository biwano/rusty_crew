use crate::hud::PlayerScore;
use crate::movable::Movable;
use crate::projectiles::Projectile;
use crate::ship::Ship;
use bevy::prelude::*;

const TARGET_HIT_POINTS: f32 = 20.0;

// Helper function to check if two positions are colliding
fn check_collision(pos1: Vec3, pos2: Vec3, collision_distance: f32) -> bool {
    let distance = pos1.distance(pos2);
    distance < collision_distance
}

// Generic collision detection for projectile-target collisions
fn check_projectile_target_collisions_with_callback<F>(
    commands: &mut Commands,
    projectiles: &Query<(Entity, &Transform, &Projectile)>,
    targets: &mut Query<(Entity, &mut Target, &Transform)>,
    collision_distance: f32,
    mut callback: F,
) where
    F: FnMut(&mut Commands, (Entity, &Transform, &Projectile), (Entity, Mut<Target>, &Transform)),
{
    for projectile in projectiles.iter() {
        let projectile_pos = projectile.1.translation;

        for target in targets.iter_mut() {
            let target_pos = target.2.translation;

            if check_collision(projectile_pos, target_pos, collision_distance) {
                callback(commands, projectile, target);
                break; // Only hit one target per projectile
            }
        }
    }
}

// Generic collision detection for ship-target collisions
fn check_ship_target_collisions_with_callback<F>(
    ships: &mut Query<&mut Ship, (With<Ship>, Without<Target>)>,
    targets: &Query<&Target, (With<Target>, Without<Ship>)>,
    ship_transforms: &Query<&Transform, (With<Ship>, Without<Target>)>,
    target_transforms: &Query<&Transform, (With<Target>, Without<Ship>)>,
    collision_distance: f32,
    mut callback: F,
) where
    F: FnMut(Mut<Ship>, &Target),
{
    for ship_transform in ship_transforms.iter() {
        let ship_pos = ship_transform.translation;

        for target_transform in target_transforms.iter() {
            let target_pos = target_transform.translation;

            if check_collision(ship_pos, target_pos, collision_distance) {
                if let Ok(ship) = ships.single_mut() {
                    if let Ok(target) = targets.single() {
                        callback(ship, target);
                    }
                }
                break; // Only process one collision per frame for simplicity
            }
        }
    }
}

#[derive(Component)]
pub struct Target {
    pub hit_points: f32,
    pub score: u32,
    pub damage: f32,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            hit_points: TARGET_HIT_POINTS,
            score: 100,   // Default score for destroying a target
            damage: 20.0, // Damage dealt to ship on collision
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

    check_projectile_target_collisions_with_callback(
        &mut commands,
        &projectiles,
        &mut targets,
        collision_distance,
        |commands,
         (projectile_entity, _projectile_transform, projectile),
         (_target_entity, mut target, _target_transform)| {
            // Apply damage to target
            target.hit_points -= projectile.damage;

            // Despawn projectile
            commands.entity(projectile_entity).despawn();
        },
    );
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
    mut player_score: ResMut<PlayerScore>,
) {
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    for (entity, target, _transform) in targets.iter() {
        if target.hit_points <= 0.0 {
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

pub fn check_ship_target_collisions(
    mut ships: Query<&mut Ship, (With<Ship>, Without<Target>)>,
    targets: Query<&Target, (With<Target>, Without<Ship>)>,
    ship_transforms: Query<&Transform, (With<Ship>, Without<Target>)>,
    target_transforms: Query<&Transform, (With<Target>, Without<Ship>)>,
) {
    let collision_distance = 0.5; // Collision detection distance

    check_ship_target_collisions_with_callback(
        &mut ships,
        &targets,
        &ship_transforms,
        &target_transforms,
        collision_distance,
        |mut ship, target| {
            // Apply damage to ship
            ship.current_health -= target.damage;
            // Ensure health doesn't go below 0
            if ship.current_health < 0.0 {
                ship.current_health = 0.0;
            }
        },
    );
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_targets).add_systems(
            Update,
            (
                check_projectile_target_collisions,
                check_ship_target_collisions,
                update_target_colors,
                despawn_dead_targets,
                despawn_out_of_bounds_targets,
            ),
        );
    }
}
