use crate::collision::{Collidable, Team};
use crate::weapons::cannon::create_cannon;
use crate::weapons::create_rocket_launcher;
use crate::weapons::weapon::{attach_weapon, fire_weapon, Weapon, WeaponMesh};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Ship;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ship).add_systems(
            Update,
            (
                update_ship_velocity,
                set_ship_rotation,
                switch_weapon_input,
                activate_weapon,
            ),
        );
    }
}

#[derive(Resource)]
pub struct SpaceshipEntity(pub Entity);

/// Removes the current weapon from a ship entity, including despawning weapon mesh
pub fn remove_weapon(
    commands: &mut Commands,
    ship_entity: Entity,
    weapon_meshes: &Query<Entity, With<WeaponMesh>>,
) {
    // Remove the Weapon component from the ship entity
    commands.entity(ship_entity).remove::<Weapon>();

    // Despawn any weapon mesh entities
    for weapon_mesh_entity in weapon_meshes.iter() {
        commands.entity(weapon_mesh_entity).despawn();
    }
}

/// Switches the weapon on a ship entity
pub fn switch_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    ship_entity: Entity,
    weapon_meshes: &Query<Entity, With<WeaponMesh>>,
    new_weapon: Weapon,
) {
    // Remove current weapon
    remove_weapon(commands, ship_entity, weapon_meshes);

    // Attach new weapon with default rotation and scale
    attach_weapon(
        commands,
        asset_server,
        scene_spawner,
        ship_entity,
        new_weapon,
        Quat::IDENTITY,
        Vec3::splat(10.0),
    );
}

pub fn setup_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Load and spawn spaceship at position (0, 0, 0), scaled to 1/100th size
    let spaceship_handle = asset_server.load("models/ships/spaceship.glb#Scene0");

    let spaceship_entity = commands
        .spawn((
            Ship,
            Collidable::new(1000.0, 100.0, Team::Player), // no damage, 100 HP, player team
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(0.01), // Scale to 1/100th size
            },
            Velocity::default(),
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.0,
            },
            RigidBody::KinematicVelocityBased,
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            SceneRoot(spaceship_handle),
            AsyncSceneCollider {
                shape: Some(ComputedColliderShape::ConvexHull),
                named_shapes: Default::default(),
            },
        ))
        .id();

    // Attach rocket launcher weapon to the ship (default weapon)
    let rocket_weapon = create_rocket_launcher(Vec3::new(0.0, 0.0, 0.0)); // Position rocket launcher at ship origin
    attach_weapon(
        &mut commands,
        &asset_server,
        &mut scene_spawner,
        spaceship_entity,
        rocket_weapon,
        Quat::IDENTITY,
        Vec3::splat(10.0),
    );

    // Store spaceship entity for movement system
    commands.insert_resource(SpaceshipEntity(spaceship_entity));
}

pub fn update_ship_velocity(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    mut query: Query<(&mut Velocity, &Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    if let Ok((mut velocity, transform)) = query.get_mut(spaceship_entity.0) {
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

        // Viewport boundary system - prevent ship from going out of bounds
        // Get window dimensions for resolution-independent calculations
        if let Ok(window) = windows.single() {
            let window_width = window.width();
            let window_height = window.height();
            let boundary_margin = 0.05; // Margin from viewport edge (5% of viewport) - kept for reference

            // Get camera for screen coordinate calculations
            if let Ok((camera, camera_global_transform)) = camera_query.single() {
                // Convert world position to screen coordinates
                if let Ok(viewport_pos) =
                    camera.world_to_viewport(camera_global_transform, transform.translation)
                {
                    // Convert normalized viewport coordinates to screen pixels
                    let screen_x = viewport_pos.x / window_width;
                    let screen_y = viewport_pos.y / window_height;

                    // Check X boundaries in screen pixels
                    if screen_x > 1.0 - boundary_margin && velocity.linvel.x > 0.0 {
                        velocity.linvel.x = 0.0; // Zero out velocity going out of bounds to the right
                    } else if screen_x < boundary_margin && velocity.linvel.x < 0.0 {
                        velocity.linvel.x = 0.0; // Zero out velocity going out of bounds to the left
                    }

                    // Check Y boundaries in screen pixels
                    if screen_y > 1.0 - boundary_margin && velocity.linvel.y < 0.0 {
                        velocity.linvel.y = 0.0; // Zero out velocity going out of bounds upward
                    } else if screen_y < boundary_margin && velocity.linvel.y > 0.0 {
                        velocity.linvel.y = 0.0; // Zero out velocity going out of bounds downward
                    }
                }
            }
        }

        // Apply acceleration to velocity
        velocity.linvel += accel_vector * time.delta_secs();
    }
}

pub fn set_ship_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = transforms.get_mut(spaceship_entity.0) {
        let rotation_speed = 2.0; // Rotation speed in radians per second
        let mut rotation_delta = 0.0;

        // Q = rotate counter-clockwise (positive Y rotation)
        if keyboard_input.pressed(KeyCode::KeyQ) {
            rotation_delta += rotation_speed * time.delta_secs();
        }
        // E = rotate clockwise (negative Y rotation)
        if keyboard_input.pressed(KeyCode::KeyE) {
            rotation_delta -= rotation_speed * time.delta_secs();
        }

        // Apply rotation around Y axis
        if rotation_delta != 0.0 {
            transform.rotate_y(rotation_delta);
        }
    }
}

pub fn switch_weapon_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    weapon_meshes: Query<Entity, With<WeaponMesh>>,
) {
    // Press 1 for cannon
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        let cannon_weapon = create_cannon(Vec3::new(0.0, 0.0, 0.0));
        switch_weapon(
            &mut commands,
            &asset_server,
            &mut scene_spawner,
            spaceship_entity.0,
            &weapon_meshes,
            cannon_weapon,
        );
    }

    // Press 2 for rocket launcher
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        let rocket_weapon = create_rocket_launcher(Vec3::new(0.0, 0.0, 0.0));
        switch_weapon(
            &mut commands,
            &asset_server,
            &mut scene_spawner,
            spaceship_entity.0,
            &weapon_meshes,
            rocket_weapon,
        );
    }
}

pub fn activate_weapon(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapons: Query<&mut Weapon>,
    spaceship_entity: Res<SpaceshipEntity>,
    transforms: Query<&Transform>,
    velocities: Query<&Velocity>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Check if space is pressed (can be held down)
    if keyboard_input.pressed(KeyCode::Space) {
        // Get the weapon attached to the spaceship
        if let Ok(mut weapon) = weapons.get_mut(spaceship_entity.0) {
            fire_weapon(
                &mut weapon,
                spaceship_entity.0,
                &transforms,
                &velocities,
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                &mut scene_spawner,
            );
        }
    }
}
