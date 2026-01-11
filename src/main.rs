mod ship;
mod starfield;
mod projectile;
mod target;
mod movable;

use bevy::{
    color::palettes::css::*,
    pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        RenderPlugin,
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
    },
};

use ship::{setup_ship, fire, set_ship_acceleration};
use starfield::{setup_starfield, move_stars, rotate_skybox};
use projectile::despawn_out_of_bounds_projectiles;
use target::{setup_targets, check_projectile_target_collisions, update_target_colors, despawn_dead_targets};
use movable::{apply_acceleration_to_movable, move_movable};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_ship, setup_starfield, setup_targets))
        .add_systems(Update, (
            set_ship_acceleration,
            apply_acceleration_to_movable,
            move_movable, 
            move_stars, 
            rotate_skybox, 
            fire, 
            despawn_out_of_bounds_projectiles, 
            check_projectile_target_collisions,
            update_target_colors,
            despawn_dead_targets,
        ))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // Set background to pure black
    commands.insert_resource(ClearColor(Color::BLACK));

    // directional light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Text used to show controls
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}
