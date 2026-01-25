use crate::collision::{Collidable, Team};
use crate::weapons::cannon::create_cannon;
use crate::weapons::create_rocket_launcher;
use crate::weapons::weapon::{Weapon, WeaponMesh};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Ship;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ship).add_systems(
            Update,
            (update_ship_velocity, set_ship_rotation, switch_weapon_input),
        );
    }
}

#[derive(Resource)]
pub struct SpaceshipEntity(pub Entity);

/// Attaches a weapon to a ship entity, including spawning the weapon mesh if available
pub fn attach_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    ship_entity: Entity,
    weapon: Weapon,
) {
    // Store position offset and mesh spawner before moving weapon
    let position_offset = weapon.weapon_position_offset;
    let mesh_spawner = weapon.mesh_spawner;

    // Add weapon component to the ship entity
    commands.entity(ship_entity).insert(weapon);

    // Spawn weapon mesh as a child of the ship using the weapon's mesh spawner
    if let Some(spawner) = mesh_spawner {
        spawner(
            commands,
            asset_server,
            scene_spawner,
            ship_entity,
            position_offset,
        );
    }
}

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

    // Attach new weapon
    attach_weapon(
        commands,
        asset_server,
        scene_spawner,
        ship_entity,
        new_weapon,
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
            Collidable::new(0.5, 1000.0, 100.0, Team::Player), // 0.5 radius, no damage, 100 HP, player team
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
    //scene_spawner.spawn_as_child(spaceship_handle, spaceship_entity);
    /*commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("models/ships/spaceship.glb#Scene0"),
    )));*/

    // Attach rocket launcher weapon to the ship (default weapon)
    let rocket_weapon = create_rocket_launcher(Vec3::new(0.0, 0.0, 0.0)); // Position rocket launcher at ship origin
    attach_weapon(
        &mut commands,
        &asset_server,
        &mut scene_spawner,
        spaceship_entity,
        rocket_weapon,
    );

    // Store spaceship entity for movement system
    commands.insert_resource(SpaceshipEntity(spaceship_entity));
}

pub fn update_ship_velocity(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    spaceship_entity: Res<SpaceshipEntity>,
    mut query: Query<&mut Velocity>,
    time: Res<Time>,
) {
    if let Ok(mut velocity) = query.get_mut(spaceship_entity.0) {
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
