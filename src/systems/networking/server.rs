use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};

use crate::{
    components::SyncedEntity,
    enums::EntityState,
    events::{
        ClientConnectedEvent, ClientDisconnectedEvent, PlayerCommand, PlayerCommandEvent,
        PlayerInputEvent,
    },
    networking::{ClientChannel, NetworkedEntities, ServerChannel},
    resources::PlayerInput,
};
pub fn server_update_system(
    mut writer_client_connected: EventWriter<ClientConnectedEvent>,
    mut writer_client_disconnected: EventWriter<ClientDisconnectedEvent>,
    mut writer_player_input: EventWriter<PlayerInputEvent>,
    mut writer_player_command: EventWriter<PlayerCommandEvent>,
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                writer_client_connected.send(ClientConnectedEvent(ServerEvent::ClientConnected {
                    client_id: *client_id,
                }));
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                writer_client_disconnected.send(ClientDisconnectedEvent(
                    ServerEvent::ClientDisconnected {
                        client_id: *client_id,
                        reason: reason.clone(),
                    },
                ));
            }
        }
    }

    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerInput = bincode::deserialize(&message).unwrap();
            writer_player_input.send(PlayerInputEvent(input, client_id.raw()));
        }

        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let command: PlayerCommand = bincode::deserialize(&message).unwrap();
            writer_player_command.send(PlayerCommandEvent(command, client_id.raw()));
        }
    }
}

pub fn server_network_sync(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform, &EntityState, &PlayerInput), With<SyncedEntity>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform, entity_state, player_input) in query.iter() {
        networked_entities.entities.push(entity);
        networked_entities
            .translations
            .push(transform.translation.into());
        networked_entities.aim_ats.push(player_input.aim.into());
        networked_entities.states.push(*entity_state)
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}
