use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Skybox;

pub fn setup_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Load and spawn skybox centered at origin (where camera is looking) and make it very large
    let skybox_handle = asset_server.load("models/environment/deep_space_skybox.glb#Scene0");
    let skybox_entity = commands
        .spawn((
            Skybox,
            Transform {
                translation: Vec3::ZERO, // Center at origin
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(100.0), // Make it way bigger (100x scale)
            },
        ))
        .id();
    scene_spawner.spawn_as_child(skybox_handle, skybox_entity);

    let mut rng = rand::thread_rng();
    let star_count = 100;
    let field_size = 50.0; // Size of the starfield cube

    // Create a small sphere mesh for stars
    let star_mesh = meshes.add(Sphere::new(0.05));

    // Spawn stars randomly distributed in a cube around the origin
    for _ in 0..star_count {
        let x = rng.gen_range(-field_size..field_size);
        let y = rng.gen_range(-field_size..field_size);
        let z = rng.gen_range(-field_size..-5.0);

        // Vary star brightness slightly
        let brightness = rng.gen_range(0.5..1.0);
        let star_color = Color::srgb(brightness, brightness, brightness);

        // Create a glowing white material for each star
        let star_material = materials.add(StandardMaterial {
            base_color: star_color,
            emissive: star_color.into(),
            ..default()
        });

        commands.spawn((
            Star,
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(star_material),
            Transform::from_translation(Vec3::new(x, y, z)),
        ));
    }
}

pub fn move_stars(mut stars: Query<&mut Transform, With<Star>>, time: Res<Time>) {
    let speed = 1.0; // Speed of star movement
    let field_size = 50.0; // Same as in setup_starfield

    for mut transform in stars.iter_mut() {
        // Move star from right to left (decrease X)
        transform.translation.x -= speed * time.delta_secs();

        // Wrap around: if star goes off the left side, move it to the right side
        if transform.translation.x < -field_size {
            transform.translation.x = field_size;
        }
    }
}

pub fn rotate_skybox(mut skybox: Query<&mut Transform, With<Skybox>>, time: Res<Time>) {
    let rotation_speed = 0.001; // Slow rotation speed (radians per second)

    for mut transform in skybox.iter_mut() {
        // Rotate around Y axis (vertical axis) for a slow spinning effect
        transform.rotate_y(rotation_speed * time.delta_secs());
    }
}

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_starfield)
            .add_systems(Update, (move_stars, rotate_skybox));
    }
}
