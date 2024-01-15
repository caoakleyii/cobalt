use bevy::ecs::event::Event;
use bevy_renet::renet::ServerEvent;

use crate::{input::resources::PlayerInput, player::events::PlayerCommand};

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
pub struct ClientSentInputEvent(pub PlayerInput, pub u64);

/**
 * Player Command Event
 *
 * A Bevy Event to handle the PlayerCommand sent from the client.
 */
#[derive(Event, Debug)]
pub struct ClientSentCommandEvent(pub PlayerCommand, pub u64);
