use crate::deck::{
    events::ShuffleEvent,
    keyword::events::{DamageEntityEvent, SpawnProjectileEvent},
};
use bevy::{ecs::event::Event, prelude::Component, utils::Uuid};
use serde::{Deserialize, Serialize};

use crate::player::events::{CreatePlayerEvent, RemovePlayerEvent};

/**
 * Server Messages
 *
 * Message types the client receives from theserver
 */
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate(CreatePlayerEvent),
    PlayerRemove(RemovePlayerEvent),
    SpawnProjectile(SpawnProjectileEvent),
    DamageEntity(DamageEntityEvent),
    Shuffle(ShuffleEvent),
}

#[derive(Event, Clone, Debug)]
pub struct ServerMessage {
    pub message_id: Uuid,
    pub message: ServerMessages,
}

impl ServerMessage {
    pub fn new(message: ServerMessages) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            message,
        }
    }
}

#[derive(Event, Clone, Debug)]
pub struct ReplayedServerMessage {
    pub message_id: Uuid,
    pub message: ServerMessages,
}

impl ReplayedServerMessage {
    pub fn new(server_message: ServerMessage) -> Self {
        Self {
            message_id: server_message.message_id,
            message: server_message.message,
        }
    }
}
