use bevy::prelude::{Entity, Event};
use serde::{Deserialize, Serialize};

/**
 *
 * A Bevy Event to inform client systems
 * an entity should be damaged, contains the corresponding
 * server message
 */
#[derive(Event, Debug, Serialize, Deserialize)]
pub struct DamageEntityEvent {
    pub entity: Entity,
    pub damage: f32,
}

/**
 *
 * A Bevy Event to inform client systems
 * a projectile should be spawned, contains the corresponding
 * server message
 */
#[derive(Event, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SpawnProjectileEvent {
    pub translation: [f32; 3],
    pub velocity: [f32; 2],
    pub projectile_type: u8,
    pub layer: u32,
    pub mask: u32,
}
