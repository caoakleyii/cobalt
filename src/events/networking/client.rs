use bevy::prelude::Event;

use crate::enums::ServerMessages;

/**
 * Player Create Event
 *
 * A Bevy Event to let inform the client
 * a player should be created, contains the corresponding
 * server message
*/
#[derive(Event, Debug)]
pub struct PlayerCreateEvent(pub ServerMessages);

/**
 *
 * A Bevy Event to infrom client systems
 * a player should be removed, contains the corresponding
 * server message
 */
#[derive(Event, Debug)]
pub struct PlayerRemoveEvent(pub ServerMessages);
