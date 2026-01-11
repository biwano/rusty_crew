use bevy::prelude::*;

#[derive(Resource)]
pub struct SpaceshipEntity(Entity);

pub fn setup_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Load and spawn spaceship at position (0, 0, 0), scaled to 1/100th size
    let spaceship_handle = asset_server.load("models/spaceship.glb#Scene0");
    let spaceship_entity = commands.spawn(Transform {
        translation: Vec3::new(0.0, 0.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.02), // Scale to 1/100th size
    }).id();
    scene_spawner.spawn_as_child(spaceship_handle, spaceship_entity);
    
    // Store spaceship entity for movement system
    commands.insert_resource(SpaceshipEntity(spaceship_entity));
}

pub fn move_spaceship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = transforms.get_mut(spaceship_entity.0) {
        let speed = 2.0;
        let mut movement = Vec3::ZERO;

        // WASD controls
        // W = up (positive Y)
        // A = left (negative X)
        // S = down (negative Y)
        // D = right (positive X)
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += speed * time.delta_secs();
        }

        transform.translation += movement;
    }
}

