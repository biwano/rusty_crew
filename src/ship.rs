use bevy::prelude::*;
use crate::movable::Movable;
use crate::weapons::cannon::create_cannon;

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
        create_cannon(), // Cannon weapon
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


