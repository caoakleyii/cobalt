use bevy::prelude::{Entity, Event};
use serde::{Deserialize, Serialize};

use crate::resources::ClientId;

/**
 * Player Create Event
 *
 * A Bevy Event to let inform the client
 * a player should be created, contains the corresponding
 * server message
*/
#[derive(Event, Debug, Serialize, Deserialize)]
pub struct PlayerCreateEvent {
    pub entity: Entity,
    pub id: ClientId,
    pub translation: [f32; 3],
}

/**
 *
 * A Bevy Event to infrom client systems
 * a player should be removed, contains the corresponding
 * server message
 */
#[derive(Event, Debug, Serialize, Deserialize)]
pub struct PlayerRemoveEvent {
    pub id: ClientId,
}

/**
 *
 * A Bevy Event to inform client systems
 * a projectile should be spawned, contains the corresponding
 * server message
 */
#[derive(Event, Debug, Serialize, Deserialize)]
pub struct SpawnProjectileEvent {
    pub translation: [f32; 3],
    pub velocity: [f32; 2],
}
