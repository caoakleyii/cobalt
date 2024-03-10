use bevy::ecs::schedule::SystemSet;

/// A System Set that runs when a client is connected
///
/// Determined by the following renet functions
/// bevy_renet::transport::client_connected()
/// bevy_renet::steam::client_disconnected(
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientConnected;

/// A System Set that runs when a local player is created
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerSet;
