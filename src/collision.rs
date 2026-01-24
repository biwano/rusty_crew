use bevy::prelude::*;

/// Represents the team affiliation of a collidable entity
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    Player = 1, // Ship and projectiles fired by ship
    Enemy = 2,  // Targets and their projectiles
}

/// A component that makes an entity collidable with other collidable entities
#[derive(Component, Clone)]
pub struct Collidable {
    /// The radius of the collision hitbox
    pub hitbox: f32,
    /// The amount of damage this entity deals on collision
    pub damage: f32,
    /// Maximum hit points this entity can have
    pub max_hit_points: f32,
    /// Current hit points remaining
    pub hit_points: f32,
    /// The team this entity belongs to
    pub team: Team,
}

impl Collidable {
    /// Create a new Collidable with the specified parameters
    pub fn new(hitbox: f32, damage: f32, max_hit_points: f32, team: Team) -> Self {
        Self {
            hitbox,
            damage,
            max_hit_points,
            hit_points: max_hit_points,
            team,
        }
    }

    /// Check if this entity is alive (has hit points remaining)
    pub fn is_alive(&self) -> bool {
        self.hit_points > 0.0
    }

    /// Apply damage to this entity
    pub fn take_damage(&mut self, damage: f32) {
        self.hit_points -= damage;
        if self.hit_points < 0.0 {
            self.hit_points = 0.0;
        }
    }
}

/// Check for collisions between all collidable entities
pub fn check_collisions(
    _commands: Commands,
    mut collidables: Query<(Entity, &Transform, &mut Collidable)>,
) {
    let entities: Vec<(Entity, Vec3, Collidable)> = collidables
        .iter()
        .map(|(entity, transform, collidable)| (entity, transform.translation, collidable.clone()))
        .collect();

    // Check collisions between all pairs
    for i in 0..entities.len() {
        let (entity_a, pos_a, collidable_a) = &entities[i];

        for j in (i + 1)..entities.len() {
            let (entity_b, pos_b, collidable_b) = &entities[j];

            // Skip collision if entities are on the same team
            if collidable_a.team == collidable_b.team {
                continue;
            }

            // Check distance collision
            let distance = pos_a.distance(*pos_b);
            let combined_hitbox = collidable_a.hitbox + collidable_b.hitbox;

            if distance < combined_hitbox {
                // Collision detected! Apply damage to both entities
                if let Ok([mut coll_a, mut coll_b]) =
                    collidables.get_many_mut([*entity_a, *entity_b])
                {
                    coll_a.2.take_damage(collidable_b.damage);
                    coll_b.2.take_damage(collidable_a.damage);
                }
            }
        }
    }
}

/// Despawn any collidable entities that have died (hit_points <= 0)
pub fn despawn_dead_collidable(mut commands: Commands, collidables: Query<(Entity, &Collidable)>) {
    for (entity, collidable) in collidables.iter() {
        if !collidable.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}

/// Plugin for managing collision detection and resolution
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_collisions, despawn_dead_collidable));
    }
}
