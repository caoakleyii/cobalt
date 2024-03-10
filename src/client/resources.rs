use std::collections::HashMap;

use bevy::{
    prelude::{Deref, Entity, Resource},
    time::Timer,
    utils::Uuid,
};
use serde::{Deserialize, Serialize};

/// A struct that holds a client id
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Deref)]
pub struct ClientId(pub u64);

/// A struct that holds the current client's id
#[derive(Debug, Resource)]
pub struct CurrentClientId(pub u64);

/// A struct that holds the server and the client's attached entity
#[derive(Debug)]
pub struct PlayerInfo {
    pub client_entity: Entity,
    pub server_entity: Entity,
}

/// A struct that holds the client's within the lobby
#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    pub players: HashMap<ClientId, PlayerInfo>,
}

/// A HashMap of Server Synced Network Entities
#[derive(Default, Resource)]
pub struct NetworkEntities(pub HashMap<Entity, Entity>);

/// A struct that holds the expiry time for replayed messages
/// This is used to remove old messages from the replay buffer
/// after a certain amount of time
#[derive(Debug, Default, Resource)]
pub struct ReplayMessageExpiry {
    pub messages: HashMap<Uuid, Timer>,
}
