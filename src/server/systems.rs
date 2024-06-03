use bevy::prelude::*;

use bevy_renet::renet::{
    RenetServer,
    ServerEvent::{self, ClientConnected, ClientDisconnected},
};

use crate::{
    asset::{
        resources::{AssetHandler, AssetsConfig},
    },
    client::resources::ClientId,
    enums::{EntityState},
    input::resources::PlayerInput,
    networking::{
        channels::{ClientChannel, ServerChannel},
        components::SyncedEntity,
        models::NetworkedEntities,
        networking::ServerMessages,
    },
    player::{
        components::{Player, Team},
        events::{CreatePlayerEvent, PlayerCommand, RemovePlayerEvent},
    },
    server::{
        events::{ClientSentCommandEvent, ClientSentInputEvent},
        resources::ServerLobby,
    },
};

use super::events::{ClientConnectedEvent, ClientDisconnectedEvent, SyncEntityEvent};

pub fn server_update_system(
    mut writer_client_connected: EventWriter<ClientConnectedEvent>,
    mut writer_client_disconnected: EventWriter<ClientDisconnectedEvent>,
    mut writer_player_input: EventWriter<ClientSentInputEvent>,
    mut writer_player_command: EventWriter<ClientSentCommandEvent>,
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
            writer_player_input.send(ClientSentInputEvent(input, client_id.raw()));
        }

        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let command: PlayerCommand = bincode::deserialize(&message).unwrap();
            writer_player_command.send(ClientSentCommandEvent(command, client_id.raw()));
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

pub fn on_demand_server_network_sync(
    mut server: ResMut<RenetServer>,
    mut reader_forced_server_sync: EventReader<SyncEntityEvent>,
    query: Query<(&Transform, &EntityState, &PlayerInput), With<SyncedEntity>>,
) {
    for sync_entity_event in reader_forced_server_sync.read() {
        let mut networked_entities = NetworkedEntities::default();

        if let Ok((transform, entity_state, player_input)) = query.get(sync_entity_event.entity) {
            networked_entities.entities.push(sync_entity_event.entity);
            networked_entities
                .translations
                .push(transform.translation.into());
            networked_entities.aim_ats.push(player_input.aim.into());
            networked_entities.states.push(*entity_state)
        }

        let sync_message = bincode::serialize(&networked_entities).unwrap();
        server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
    }
}

// TODO: Simplify the connect and write to a "client_connected" event; then a seperate system to handle the player create event within Player Plugin
pub fn client_connected_to_server(
    _commands: Commands,
    mut writer_create_player: EventWriter<CreatePlayerEvent>,
    mut reader_client_connected: EventReader<ClientConnectedEvent>,
    _lobby: ResMut<ServerLobby>,
    _server: ResMut<RenetServer>,
    _asset_handler: Res<AssetHandler>,
    _asset_config: Res<AssetsConfig>,
    _players: Query<(Entity, &Player, &Transform, &Team)>,
) {
    for client_connected in reader_client_connected.read() {
        match client_connected.0 {
            ClientConnected { client_id } => {
                writer_create_player.send(CreatePlayerEvent {
                    entity: Entity::from_raw(0),
                    id: ClientId(client_id.raw()),
                    translation: [0.0, 0.0, 0.0],
                    team: 0,
                });
            }
            _ => {}
        }
    }
}

pub fn client_disconnected(
    mut commands: Commands,
    mut reader_client_disconnected: EventReader<ClientDisconnectedEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
) {
    for client_disconnected in reader_client_disconnected.read() {
        match client_disconnected.0 {
            ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected. {}", client_id, reason);

                if let Some(player_entity) = lobby.players.remove(&client_id.raw()) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove(RemovePlayerEvent {
                        id: ClientId(client_id.raw()),
                    }))
                    .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            _ => println!("Unexpected server event in client disconnect event stream."),
        }
    }
}
