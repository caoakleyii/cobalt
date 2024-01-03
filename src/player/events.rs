use bevy::{
    ecs::entity::Entity,
    prelude::{Component, Event, Vec2},
};
use serde::{Deserialize, Serialize};

use crate::client::resources::ClientId;

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub enum PlayerCommand {
    UseEquipment { cast_at: Vec2 },
    // ChangeEquipment { equipment: Entity },
}

/**
 * Player Create Event
 *
 * A Bevy Event to let inform the client
 * a player should be created, contains the corresponding
 * server message
*/
#[derive(Event, Debug, Serialize, Deserialize)]
pub struct CreatePlayerEvent {
    pub entity: Entity,
    pub id: ClientId,
    pub translation: [f32; 3],
    pub team: u32,
}

/**
 *
 * A Bevy Event to infrom client systems
 * a player should be removed, contains the corresponding
 * server message
 */
#[derive(Event, Debug, Serialize, Deserialize)]
pub struct RemovePlayerEvent {
    pub id: ClientId,
}
