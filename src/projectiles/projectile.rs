use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self { damage: 10.0 }
    }
}

pub fn despawn_out_of_bounds_projectiles(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform), With<Projectile>>,
) {
    let boundary = 50.0; // Distance from origin before despawning

    for (entity, transform) in projectiles.iter() {
        let pos = transform.translation;
        // Despawn if projectile is too far in any direction
        if pos.x.abs() > boundary || pos.y.abs() > boundary || pos.z.abs() > boundary {
            commands.entity(entity).despawn();
        }
    }
}
