mod collision;
mod hud;
mod projectiles;
mod ship;
mod starfield;
mod target;
mod weapons;

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use collision::CollisionPlugin;
use hud::HudPlugin;
use projectiles::ProjectilePlugin;
use ship::ShipPlugin;
use starfield::StarfieldPlugin;
use target::TargetPlugin;
use weapons::WeaponPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //       .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CollisionPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(ShipPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(StarfieldPlugin)
        .add_plugins(TargetPlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
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
}
