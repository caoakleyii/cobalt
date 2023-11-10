use bevy::prelude::Event;
use bevy_renet::renet::ServerEvent;

use crate::resources::PlayerInput;

/**
 * Client Connected Event
 *
 * A Bevy Event to inform the server
 * a new client has been connected. Contains
 * the corresponding server event.
 */
#[derive(Event, Debug)]
pub struct ClientConnectedEvent(pub ServerEvent);

/**
 * Client Disconnected Event
 *
 * A Bevy Event to inform server systems
 * a client has been disconnected. Contains
 * the corresponding server event.
 */
#[derive(Event, Debug)]
pub struct ClientDisconnectedEvent(pub ServerEvent);

/**
 * Player Input Event
 *
 * A Bevy Event to handle the PlayerInput sent from the client.
 */
#[derive(Event, Debug)]
pub struct PlayerInputEvent(pub PlayerInput, pub u64);
