use bevy::prelude::*;
use bevy_renet::renet::{RenetServer, ServerEvent};

use crate::{
    deck::card::equipment::components::ServerEquipmentBundle,
    enums::EntityState,
    networking::{
        channels::{ClientChannel, ServerChannel},
        components::SyncedEntity,
        models::NetworkedEntities,
    },
    player::events::PlayerCommand,
    resources::PlayerInput,
    server::events::{ClientSentCommandEvent, ClientSentInputEvent},
};

use super::events::{ClientConnectedEvent, ClientDisconnectedEvent};

use bevy_2d_collisions::components::CollisionGroup;
use bevy_renet::renet::ServerEvent::{ClientConnected, ClientDisconnected};

use crate::asset::enums::{Equipment, Sprites};
use crate::client::resources::ClientId;
use crate::components::{Player, ServerPlayerBundle, Team};
use crate::enums::CollisionGroups;
use crate::networking::networking::ServerMessages;
use crate::player::events::{CreatePlayerEvent, RemovePlayerEvent};

use crate::asset::resources::{AssetHandler, AssetsConfig};

use crate::server::resources::ServerLobby;

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

// TODO: Simplify the connect and write to a "client_connected" event; then a seperate system to handle the player create event within Player Plugin
pub fn client_connected_to_server(
    mut commands: Commands,
    mut reader_client_connected: EventReader<ClientConnectedEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    asset_handler: Res<AssetHandler>,
    asset_config: Res<AssetsConfig>,
    players: Query<(Entity, &Player, &Transform, &Team)>,
) {
    for client_connected in reader_client_connected.read() {
        match client_connected.0 {
            ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                // initialize the newly connected client with the current state of the players in the game
                for (entity, player, transform, p_team) in &players {
                    let translation: [f32; 3] = transform.translation.into();
                    let message =
                        bincode::serialize(&ServerMessages::PlayerCreate(CreatePlayerEvent {
                            id: ClientId(player.id.raw()),
                            entity,
                            translation,
                            team: (**p_team).into(),
                        }))
                        .unwrap();
                    server.send_message(client_id, ServerChannel::ServerMessages, message);
                }

                // spawn the player on the server

                let character_type = Sprites::Skeleton;
                // Retrieve character assets from the already loaded resources
                let (_texture, _animations, hitbox_config) = asset_handler
                    .textures
                    .get(&character_type)
                    .expect("unexpected character requested.");

                let hitbox_config = hitbox_config.expect(&format!(
                    "Requested character does not have a hitbox: {:?}",
                    character_type
                ));

                // TODO: Change this, curently just auto balancing teams
                let team = if lobby.players.len() % 2 == 0 {
                    CollisionGroups::TeamAlpha as u32
                } else {
                    CollisionGroups::TeamBravo as u32
                };

                let spawn_point = Vec3::new(0.0, 0.0, 0.0);
                let player_entity = commands
                    .spawn(ServerPlayerBundle::new(
                        client_id,
                        Transform::from_translation(spawn_point.clone()),
                        Vec2::new(hitbox_config.width, hitbox_config.height),
                        CollisionGroup {
                            layer: CollisionGroups::Player as u32 | team,
                            mask: 0,
                        },
                        Team(team.into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn(ServerEquipmentBundle::new(
                            asset_config
                                .stats
                                .equipment
                                .get(&Equipment::AK47)
                                .expect("Could not find AK47 in equipment config.")
                                .into(),
                        ));
                    })
                    .id();

                lobby.players.insert(client_id.raw(), player_entity);

                // send the player entity to the clients
                let message =
                    bincode::serialize(&ServerMessages::PlayerCreate(CreatePlayerEvent {
                        id: ClientId(client_id.raw()),
                        entity: player_entity,
                        translation: spawn_point.to_array(),
                        team,
                    }))
                    .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
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
