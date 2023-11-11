use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent::{ClientConnected, ClientDisconnected};

use crate::components::{
    Animator, Controllable, EquipmentBundle, Player, PlayerBundle, PlayerCamera, ServerPlayerBundle,
};
use crate::enums::{Character, Equipment, ServerMessages};
use crate::events::{
    ClientConnectedEvent, ClientDisconnectedEvent, PlayerCreateEvent, PlayerRemoveEvent,
};
use crate::networking::ServerChannel;
use crate::resources::{
    AssetHandler, AssetsConfig, ClientId, ClientLobby, CurrentClientId, NetworkEntities,
    PlayerInfo, ServerLobby,
};

pub fn player_create_system(
    mut commands: Commands,
    mut reader_player_create: EventReader<PlayerCreateEvent>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkEntities>,
    client_id: Res<CurrentClientId>,
    asset_handler: Res<AssetHandler>,
    asset_config: Res<AssetsConfig>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: Query<&mut Window>,
) {
    for player_create in reader_player_create.iter() {
        match player_create.0 {
            ServerMessages::PlayerCreate {
                entity,
                id,
                translation,
            } => {
                println!("Player {} connected.", id.0);

                // TODO: Move this to a better camera system that allows for targets
                // * ideally follow current player or x,y,z point
                if id.0 == client_id.0 {
                    let mut camera_bundle = Camera2dBundle::default();
                    camera_bundle.projection.scale = 0.5;
                    commands.spawn((camera_bundle, PlayerCamera));
                }

                // Retrieve character assets from the already loaded resources
                let (texture, animations) = asset_handler
                    .character_textures
                    .get(&Character::Skeleton)
                    .expect("unexpected character requested.");

                // Spawn Player
                let mut player_entity = commands.spawn(PlayerBundle::new(
                    id,
                    Animator::import(animations),
                    texture_atlases.add(texture.clone()),
                    Transform::from_xyz(translation[0], translation[1], translation[2]),
                ));

                // if this is the client player, give them control
                if id.0 == client_id.0 {
                    player_entity.insert(Controllable);
                    let mut window = windows.single_mut();
                    window.cursor.icon = bevy::window::CursorIcon::Crosshair;
                }

                // Add player to network mapping
                let player_info = PlayerInfo {
                    server_entity: entity,
                    client_entity: player_entity.id(),
                };

                lobby.players.insert(id, player_info);
                network_mapping.0.insert(entity, player_entity.id());

                let player_entity = player_entity.id();

                // Spawn Equipment
                commands
                    .spawn(EquipmentBundle::new(
                        asset_config
                            .stats
                            .equipment
                            .get(&Equipment::Rifle)
                            .expect("Could not find rifle in equipment config.")
                            .into(),
                    ))
                    .set_parent(player_entity);
            }
            _ => {
                println!("Received unknown message from server.");
            }
        }
    }
}

pub fn player_remove_system(
    mut commands: Commands,
    mut reader_player_remove: EventReader<PlayerRemoveEvent>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkEntities>,
) {
    for player_remove in reader_player_remove.iter() {
        match player_remove.0 {
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id.0);

                if let Some(player_info) = lobby.players.remove(&id) {
                    commands.entity(player_info.client_entity).despawn();
                    network_mapping.0.remove(&player_info.server_entity);
                }
            }
            _ => println!("Unexpected server event in player remove event stream."),
        }
    }
}

pub fn client_connected_system(
    mut commands: Commands,
    mut reader_client_connected: EventReader<ClientConnectedEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    asset_config: Res<AssetsConfig>,
    players: Query<(Entity, &Player, &Transform)>,
) {
    for client_connected in reader_client_connected.iter() {
        match client_connected.0 {
            ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                // initialize the newly connected client with the current state of the players in the game
                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(client_id, ServerChannel::ServerMessages, message);
                }

                // spawn the player on the server
                let spawn_point = Vec3::new(0.0, 0.0, 0.0);
                let player_entity = commands
                    .spawn(ServerPlayerBundle::new(
                        ClientId(client_id),
                        Transform::from_translation(spawn_point.clone()),
                    ))
                    .with_children(|parent| {
                        parent.spawn(EquipmentBundle::new(
                            asset_config
                                .stats
                                .equipment
                                .get(&Equipment::Rifle)
                                .expect("Could not find rifle in equipment config.")
                                .into(),
                        ));
                    })
                    .id();

                lobby.players.insert(client_id, player_entity);

                // send the player entity to the client
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: ClientId(client_id),
                    entity: player_entity,
                    translation: spawn_point.to_array(),
                })
                .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            _ => {}
        }
    }
}

pub fn client_disconnected_system(
    mut commands: Commands,
    mut reader_client_disconnected: EventReader<ClientDisconnectedEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
) {
    for client_disconnected in reader_client_disconnected.iter() {
        match client_disconnected.0 {
            ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected. {}", client_id, reason);

                if let Some(player_entity) = lobby.players.remove(&client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message = bincode::serialize(&ServerMessages::PlayerRemove {
                    id: ClientId(client_id),
                })
                .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            _ => println!("Unexpected server event in client disconnect event stream."),
        }
    }
}
