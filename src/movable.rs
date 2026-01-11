use bevy::prelude::*;

#[derive(Component)]
pub struct Movable {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub damping: f32,
}

impl Movable {
    pub fn new(velocity: Vec3, acceleration: Vec3, damping: f32) -> Self {
        Self {
            velocity,
            acceleration,
            damping,
        }
    }
    
    pub fn zero(damping: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            damping,
        }
    }
    
    pub fn with_velocity(velocity: Vec3, damping: f32) -> Self {
        Self {
            velocity,
            acceleration: Vec3::ZERO,
            damping,
        }
    }
}

pub fn apply_acceleration_to_movable(
    mut movables: Query<&mut Movable>,
    time: Res<Time>,
) {
    for mut movable in movables.iter_mut() {
        // Store acceleration and damping before modifying velocity
        let acceleration = movable.acceleration;
        let damping = movable.damping;
        
        // Apply acceleration to velocity
        movable.velocity += acceleration * time.delta_secs();
        
        // Apply damping/friction
        movable.velocity *= damping;
    }
}

pub fn move_movable(
    mut transforms: Query<&mut Transform>,
    movables: Query<&Movable>,
    time: Res<Time>,
) {
    for (mut transform, movable) in transforms.iter_mut().zip(movables.iter()) {
        // Update position based on velocity
        transform.translation += movable.velocity * time.delta_secs();
    }
}

