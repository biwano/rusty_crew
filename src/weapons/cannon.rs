use crate::weapons::weapon::Weapon;
use crate::projectiles::spawn_cannon_ball;

/// Creates a cannon weapon with default settings
pub fn create_cannon() -> Weapon {
    Weapon::new(0.5, spawn_cannon_ball) // Weapon with 0.5 second cooldown, uses cannon ball projectiles
}
