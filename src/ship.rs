use bevy::prelude::*;
use crate::projectile::Projectile;
use crate::movable::Movable;

#[derive(Resource)]
pub struct SpaceshipEntity(pub Entity);

pub fn setup_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Load and spawn spaceship at position (0, 0, 0), scaled to 1/100th size
    let spaceship_handle = asset_server.load("models/spaceship.glb#Scene0");
    let spaceship_entity = commands.spawn((
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(0.01), // Scale to 1/100th size
        },
        Movable::zero(0.95), // Start with zero velocity and acceleration, damping of 0.95
    )).id();
    scene_spawner.spawn_as_child(spaceship_handle, spaceship_entity);
    
    // Store spaceship entity for movement system
    commands.insert_resource(SpaceshipEntity(spaceship_entity));
}

pub fn set_ship_acceleration(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    mut movables: Query<&mut Movable>,
) {
    if let Ok(mut movable) = movables.get_mut(spaceship_entity.0) {
        let acceleration_rate = 5.0; // Acceleration rate
        let mut accel_vector = Vec3::ZERO;

        // WASD controls - set acceleration direction
        // W = up (positive Y)
        // A = left (negative X)
        // S = down (negative Y)
        // D = right (positive X)
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            accel_vector.y += acceleration_rate;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            accel_vector.x -= acceleration_rate;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            accel_vector.y -= acceleration_rate;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            accel_vector.x += acceleration_rate;
        }

        // Set acceleration vector
        movable.acceleration = accel_vector;
    }
}

pub fn fire(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    transforms: Query<&Transform>,
    movables: Query<&Movable>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Check if space was just pressed (not held)
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Get spaceship position and velocity
        if let (Ok(spaceship_transform), Ok(ship_movable)) = (transforms.get(spaceship_entity.0), movables.get(spaceship_entity.0)) {
            // Create a small sphere for the projectile
            let projectile_mesh = meshes.add(Sphere::new(0.03));
            let projectile_material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 1.0), // Blue projectile
                emissive: Color::srgb(1.0, 1.0, 0.0).into(),
                ..default()
            });
            
            // Calculate projectile velocity: ship velocity + forward velocity (to the right)
            let forward_speed = 10.0;
            let projectile_velocity = ship_movable.velocity + Vec3::new(forward_speed, 0.0, 0.0);
            
            // Spawn projectile at spaceship position with inherited velocity
            // Projectiles have no damping (damping = 1.0) and no acceleration so they maintain constant velocity
            commands.spawn((
                Projectile::default(),
                Movable::with_velocity(projectile_velocity, 1.0),
                Mesh3d(projectile_mesh),
                MeshMaterial3d(projectile_material),
                Transform::from_translation(spaceship_transform.translation),
            ));
        }
    }
}

