use crate::collision::{Collidable, Team};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Type alias for projectile spawner functions
pub type ProjectileSpawner = fn(
    &mut Commands,
    &mut ResMut<Assets<Mesh>>,
    &mut ResMut<Assets<StandardMaterial>>,
    &Res<AssetServer>,
    &mut ResMut<SceneSpawner>,
    Vec3, // position
    Vec3, // velocity
    Quat, // rotation
    Team, // team
);

/// Type alias for mesh spawner functions
pub type MeshSpawner = fn(
    &mut Commands,
    &Res<AssetServer>,
    &mut ResMut<SceneSpawner>,
    Entity, // parent entity
    Vec3,   // translation relative to parent
    Quat,   // rotation relative to parent
    Vec3,   // scale
);

#[derive(Component)]
pub struct WeaponMesh;

#[derive(Component)]
pub struct Weapon {
    pub fire_cooldown_duration: f32,
    pub cooldown_timer: f32,
    pub projectile_spawner: Option<ProjectileSpawner>, // Optional projectile spawner function
    pub mesh_spawner: Option<MeshSpawner>,             // Optional mesh spawner function
    pub weapon_position_offset: Vec3, // Offset from ship where weapon is positioned
    pub projectile_spawn_offset: Vec3, // Offset from weapon position where projectiles spawn
    pub projectile_spawn_speed_vector: Vec3, // Base speed vector for projectiles (before rotation)
    pub weapon_rotation: Quat,        // Rotation of the weapon relative to the ship
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            fire_cooldown_duration: 1.0, // Default 1 second cooldown
            cooldown_timer: 0.0,
            projectile_spawner: None,
            mesh_spawner: None,
            weapon_position_offset: Vec3::ZERO, // Default: weapon at ship origin
            projectile_spawn_offset: Vec3::ZERO, // Default: spawn at weapon position
            projectile_spawn_speed_vector: Vec3::new(10.0, 0.0, 0.0), // Default: 10 units forward
            weapon_rotation: Quat::IDENTITY,
        }
    }

    pub fn with_fire_cooldown(mut self, duration: f32) -> Self {
        self.fire_cooldown_duration = duration;
        self
    }

    pub fn with_projectile_spawner(mut self, spawner: ProjectileSpawner) -> Self {
        self.projectile_spawner = Some(spawner);
        self
    }

    pub fn with_mesh_spawner(mut self, spawner: MeshSpawner) -> Self {
        self.mesh_spawner = Some(spawner);
        self
    }

    pub fn with_weapon_position_offset(mut self, offset: Vec3) -> Self {
        self.weapon_position_offset = offset;
        self
    }

    pub fn with_projectile_spawn_offset(mut self, offset: Vec3) -> Self {
        self.projectile_spawn_offset = offset;
        self
    }

    pub fn with_projectile_spawn_speed_vector(mut self, speed_vector: Vec3) -> Self {
        self.projectile_spawn_speed_vector = speed_vector;
        self
    }

    pub fn can_fire(&self) -> bool {
        self.cooldown_timer <= 0.0
    }

    pub fn start_cooldown(&mut self) {
        self.cooldown_timer = self.fire_cooldown_duration;
    }
}

/// Attaches a weapon to an entity, including spawning the weapon mesh if available
pub fn attach_weapon(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
    entity: Entity,
    mut weapon: Weapon,
    rotation: Quat,
    scale: Vec3,
) {
    // Store weapon rotation in the weapon component
    weapon.weapon_rotation = rotation;

    // Store position offset and mesh spawner before moving weapon
    let position_offset = weapon.weapon_position_offset;
    let mesh_spawner = weapon.mesh_spawner;

    // Add weapon component to the entity
    commands.entity(entity).insert(weapon);

    // Spawn weapon mesh as a child of the entity using the weapon's mesh spawner
    if let Some(spawner) = mesh_spawner {
        spawner(
            commands,
            asset_server,
            scene_spawner,
            entity,
            position_offset,
            rotation,
            scale,
        );
    }
}

pub fn update_weapon_cooldowns(mut weapons: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in weapons.iter_mut() {
        if weapon.cooldown_timer > 0.0 {
            weapon.cooldown_timer -= time.delta_secs();
            if weapon.cooldown_timer < 0.0 {
                weapon.cooldown_timer = 0.0;
            }
        }
    }
}

pub fn fire_weapon(
    weapon: &mut Weapon,
    owner_entity: Entity,
    transforms: &Query<&Transform>,
    velocities: &Query<&Velocity>,
    collidables: &Query<&Collidable>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    scene_spawner: &mut ResMut<SceneSpawner>,
) {
    // Check if weapon can fire (cooldown has passed)
    if weapon.can_fire() {
        // Get owner position, velocity and team
        if let (Ok(owner_transform), Ok(owner_velocity), Ok(owner_collidable)) = (
            transforms.get(owner_entity),
            velocities.get(owner_entity),
            collidables.get(owner_entity),
        ) {
            // Calculate projectile velocity: ship velocity + weapon's spawn speed vector (rotated with ship and weapon)
            let combined_rotation = owner_transform.rotation * weapon.weapon_rotation;
            let forward_direction = combined_rotation * weapon.projectile_spawn_speed_vector;
            let projectile_velocity = owner_velocity.linvel + forward_direction;

            // Calculate weapon position and projectile spawn position (rotated with ship)
            let rotated_weapon_offset = owner_transform.rotation * weapon.weapon_position_offset;
            let rotated_projectile_offset = combined_rotation * weapon.projectile_spawn_offset;

            let weapon_position = owner_transform.translation + rotated_weapon_offset;
            let projectile_position = weapon_position + rotated_projectile_offset;

            // Spawn projectile using the weapon's projectile spawner
            if let Some(spawner) = weapon.projectile_spawner {
                spawner(
                    commands,
                    meshes,
                    materials,
                    asset_server,
                    scene_spawner,
                    projectile_position,
                    projectile_velocity,
                    combined_rotation,
                    owner_collidable.team,
                );
            }

            // Start cooldown
            weapon.start_cooldown();
        }
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_weapon_cooldowns);
    }
}
