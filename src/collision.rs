use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Represents the team affiliation of a collidable entity
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    Player = 1, // Ship and projectiles fired by ship
    Enemy = 2,  // Enemies and their projectiles
}

/// A component that makes an entity collidable with other collidable entities
#[derive(Component, Clone)]
pub struct Collidable {
    /// The amount of damage this entity deals on collision
    pub damage: f32,
    /// Maximum hit points this entity can have
    pub max_hit_points: f32,
    /// Current hit points remaining
    pub hit_points: f32,
    /// The team this entity belongs to
    pub team: Team,
}

/// A marker component for entities that should not be automatically despawned when they die
#[derive(Component)]
pub struct Persistent;

impl Collidable {
    /// Create a new Collidable with the specified parameters
    pub fn new(damage: f32, max_hit_points: f32, team: Team) -> Self {
        Self {
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

/// Check for collisions using Rapier collision events
pub fn handle_collision_events(
    mut collision_events: MessageReader<CollisionEvent>,
    mut collidables: Query<&mut Collidable>,
    parents: Query<&ChildOf>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            let root_a = find_collidable_root(*e1, &collidables, &parents);
            let root_b = find_collidable_root(*e2, &collidables, &parents);

            if let (Some(entity_a), Some(entity_b)) = (root_a, root_b) {
                if entity_a == entity_b {
                    continue;
                }

                // Try to get both collidables. If one of them is missing, it's not a collision we care about
                if let Ok([mut coll_a, mut coll_b]) = collidables.get_many_mut([entity_a, entity_b])
                {
                    // Skip collision if entities are on the same team
                    if coll_a.team == coll_b.team {
                        continue;
                    }
                    println!(
                        "Collision between Team::{:?} and Team::{:?}",
                        coll_a.team, coll_b.team
                    );
                    println!("entity_a: {:?}, entity_b: {:?}", entity_a, entity_b);

                    // Apply damage to both entities based on the other's damage
                    let damage_a = coll_a.damage;
                    let damage_b = coll_b.damage;

                    coll_a.take_damage(damage_b);
                    coll_b.take_damage(damage_a);
                }
            }
        }
    }
}

/// Propagates physics settings from parent to children with colliders.
/// This is useful when using AsyncSceneCollider, as it creates colliders on children
/// but doesn't automatically copy ActiveEvents or ActiveCollisionTypes from the parent.
/// This system walks up the entity hierarchy to find the nearest ancestor with physics settings.
pub fn propagate_physics_settings(
    mut commands: Commands,
    physics_settings: Query<(&ActiveEvents, &ActiveCollisionTypes)>,
    child_query: Query<Entity, (With<Collider>, Without<ActiveEvents>)>,
    parents: Query<&ChildOf>,
) {
    for child_entity in child_query.iter() {
        let mut current = child_entity;
        // Walk up the hierarchy to find an entity with physics settings
        while let Ok(child_of) = parents.get(current) {
            let parent = child_of.parent();
            if let Ok((active_events, active_types)) = physics_settings.get(parent) {
                commands
                    .entity(child_entity)
                    .insert((*active_events, *active_types));
                println!(
                    "Propagated physics settings from ancestor {:?} to child collider {:?}",
                    parent, child_entity
                );
                break;
            }
            current = parent;
        }
    }
}

/// Helper function to find the ancestor entity that has the Collidable component
fn find_collidable_root(
    entity: Entity,
    collidables: &Query<&mut Collidable>,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    let mut current = entity;
    loop {
        if collidables.contains(current) {
            return Some(current);
        }
        if let Ok(parent) = parents.get(current) {
            current = parent.parent();
        } else {
            return None;
        }
    }
}

/// Despawn any collidable entities that have died (hit_points <= 0)
pub fn despawn_dead_collidable(
    mut commands: Commands,
    collidables: Query<(Entity, &Collidable), Without<Persistent>>,
) {
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
        app.add_systems(
            Update,
            (
                handle_collision_events,
                propagate_physics_settings,
                despawn_dead_collidable,
            ),
        );
    }
}
