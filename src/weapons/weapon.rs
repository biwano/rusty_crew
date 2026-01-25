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
);

/// Type alias for mesh spawner functions
pub type MeshSpawner = fn(
    &mut Commands,
    &Res<AssetServer>,
    &mut ResMut<SceneSpawner>,
    Entity, // parent entity
    Vec3,   // translation relative to parent
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

    #[allow(dead_code)]
    pub fn set_fire_cooldown(&mut self, duration: f32) -> &mut Self {
        self.fire_cooldown_duration = duration;
        self
    }

    #[allow(dead_code)]
    pub fn set_projectile_spawner(&mut self, spawner: ProjectileSpawner) -> &mut Self {
        self.projectile_spawner = Some(spawner);
        self
    }

    #[allow(dead_code)]
    pub fn set_mesh_spawner(&mut self, spawner: MeshSpawner) -> &mut Self {
        self.mesh_spawner = Some(spawner);
        self
    }

    #[allow(dead_code)]
    pub fn set_weapon_position_offset(&mut self, offset: Vec3) -> &mut Self {
        self.weapon_position_offset = offset;
        self
    }

    #[allow(dead_code)]
    pub fn set_projectile_spawn_offset(&mut self, offset: Vec3) -> &mut Self {
        self.projectile_spawn_offset = offset;
        self
    }

    #[allow(dead_code)]
    pub fn set_projectile_spawn_speed_vector(&mut self, speed_vector: Vec3) -> &mut Self {
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

pub fn activate_weapon(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapons: Query<&mut Weapon>,
    spaceship_entity: Res<crate::ship::SpaceshipEntity>,
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
            // Check if weapon can fire (cooldown has passed)
            if weapon.can_fire() {
                // Get spaceship position and velocity
                if let (Ok(spaceship_transform), Ok(ship_velocity)) = (
                    transforms.get(spaceship_entity.0),
                    velocities.get(spaceship_entity.0),
                ) {
                    // Calculate projectile velocity: ship velocity + weapon's spawn speed vector (rotated with ship)
                    let forward_direction =
                        spaceship_transform.rotation * weapon.projectile_spawn_speed_vector;
                    let projectile_velocity = ship_velocity.linvel + forward_direction;

                    // Calculate weapon position and projectile spawn position (rotated with ship)
                    let rotated_weapon_offset =
                        spaceship_transform.rotation * weapon.weapon_position_offset;
                    let rotated_projectile_offset =
                        spaceship_transform.rotation * weapon.projectile_spawn_offset;

                    let weapon_position = spaceship_transform.translation + rotated_weapon_offset;
                    let projectile_position = weapon_position + rotated_projectile_offset;

                    // Spawn projectile using the weapon's projectile spawner
                    if let Some(spawner) = weapon.projectile_spawner {
                        spawner(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            &asset_server,
                            &mut scene_spawner,
                            projectile_position,
                            projectile_velocity,
                            spaceship_transform.rotation,
                        );
                    }

                    // Start cooldown
                    weapon.start_cooldown();
                }
            }
        }
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_weapon_cooldowns, activate_weapon));
    }
}
