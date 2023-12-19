use std::collections::HashMap;

use bevy::prelude::{Deref, Entity, Resource, SystemSet};
use serde::{Deserialize, Serialize};

/// A System Set that runs when a client is connected
///
/// Determined by the following renet functions
/// bevy_renet::transport::client_connected()
/// bevy_renet::steam::client_disconnected(
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

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
