use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy_health_bar::ProgressBarBundle;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent::{ClientConnected, ClientDisconnected};

use crate::components::{
    Animator, Controllable, EquipmentBundle, Player, PlayerBundle, PlayerCamera,
    ServerEquipmentBundle, ServerPlayerBundle,
};
use crate::enums::{Equipment, ServerMessages, Sprites};
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
    asset_server: Res<AssetServer>,
) {
    for player_create_event in reader_player_create.read() {
        println!("Player {} connected.", player_create_event.id.0);

        // TODO: Move this to a better camera system that allows for targets
        // * ideally follow current player or x,y,z point
        if player_create_event.id.0 == client_id.0 {
            let mut camera_bundle = Camera2dBundle::default();
            camera_bundle.projection.scale = 0.5;
            commands.spawn((camera_bundle, PlayerCamera));
        }

        // Retrieve character assets from the already loaded resources
        let (texture, animations) = asset_handler
            .textures
            .get(&Sprites::Skeleton)
            .expect("unexpected character requested.");

        // Spawn Player
        let mut player_entity = commands.spawn(PlayerBundle::new(
            bevy_renet::renet::ClientId::from_raw(*player_create_event.id),
            Animator::import(animations),
            texture_atlases.add(texture.clone()),
            Transform::from_xyz(
                player_create_event.translation[0],
                player_create_event.translation[1],
                player_create_event.translation[2],
            ),
        ));

        // if this is the client player, give them control
        if player_create_event.id.0 == client_id.0 {
            player_entity.insert(Controllable);
            let mut window = windows.single_mut();
            window.cursor.icon = bevy::window::CursorIcon::Crosshair;
        }

        // Add player to network mapping
        let player_info = PlayerInfo {
            server_entity: player_create_event.entity,
            client_entity: player_entity.id(),
        };

        lobby.players.insert(player_create_event.id, player_info);
        network_mapping
            .0
            .insert(player_create_event.entity, player_entity.id());

        let player_entity = player_entity.id();

        // Retrieve character assets from the already loaded resources
        let (texture, animations) = asset_handler
            .textures
            .get(&Sprites::AK47)
            .expect("unexpected character requested.");

        // Spawn Equipment
        commands
            .spawn(EquipmentBundle::new(
                asset_config
                    .stats
                    .equipment
                    .get(&Equipment::Rifle)
                    .expect("Could not find rifle in equipment config.")
                    .into(),
                Animator::import(animations),
                texture_atlases.add(texture.clone()),
                Transform::from_xyz(5.0, -1.5, 0.0),
            ))
            .set_parent(player_entity);

        // Spawn Health Bar
        let transform = Transform::from_xyz(-15.0, 19.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5));
        commands
            .spawn(
                ProgressBarBundle::new(100.0, asset_server.load("ui/health_bar.png"))
                    .with_transform(transform),
            )
            .set_parent(player_entity);
    }
}

pub fn player_remove_system(
    mut commands: Commands,
    mut reader_player_remove: EventReader<PlayerRemoveEvent>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkEntities>,
) {
    for player_remove_event in reader_player_remove.read() {
        println!("Player {} disconnected.", player_remove_event.id.0);

        if let Some(player_info) = lobby.players.remove(&player_remove_event.id) {
            commands.entity(player_info.client_entity).despawn();
            network_mapping.0.remove(&player_info.server_entity);
        }
    }
}

pub fn client_connected_to_server_system(
    mut commands: Commands,
    mut reader_client_connected: EventReader<ClientConnectedEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    asset_config: Res<AssetsConfig>,
    players: Query<(Entity, &Player, &Transform)>,
) {
    for client_connected in reader_client_connected.read() {
        match client_connected.0 {
            ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                // initialize the newly connected client with the current state of the players in the game
                for (entity, player, transform) in &players {
                    let translation: [f32; 3] = transform.translation.into();
                    let message =
                        bincode::serialize(&ServerMessages::PlayerCreate(PlayerCreateEvent {
                            id: ClientId(player.id.raw()),
                            entity,
                            translation,
                        }))
                        .unwrap();
                    server.send_message(client_id, ServerChannel::ServerMessages, message);
                }

                // spawn the player on the server
                let spawn_point = Vec3::new(0.0, 0.0, 0.0);
                let player_entity = commands
                    .spawn(ServerPlayerBundle::new(
                        client_id,
                        Transform::from_translation(spawn_point.clone()),
                    ))
                    .with_children(|parent| {
                        parent.spawn(ServerEquipmentBundle::new(
                            asset_config
                                .stats
                                .equipment
                                .get(&Equipment::Rifle)
                                .expect("Could not find rifle in equipment config.")
                                .into(),
                        ));
                    })
                    .id();

                lobby.players.insert(client_id.raw(), player_entity);

                // send the player entity to the client
                let message =
                    bincode::serialize(&ServerMessages::PlayerCreate(PlayerCreateEvent {
                        id: ClientId(client_id.raw()),
                        entity: player_entity,
                        translation: spawn_point.to_array(),
                    }))
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
    for client_disconnected in reader_client_disconnected.read() {
        match client_disconnected.0 {
            ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected. {}", client_id, reason);

                if let Some(player_entity) = lobby.players.remove(&client_id.raw()) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove(PlayerRemoveEvent {
                        id: ClientId(client_id.raw()),
                    }))
                    .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            _ => println!("Unexpected server event in client disconnect event stream."),
        }
    }
}
