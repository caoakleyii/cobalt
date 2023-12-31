use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use bevy_2d_collisions::components::CollisionGroup;
use bevy_health_bar::ProgressBarBundle;
use bevy_renet::renet::RenetServer;
use bevy_renet::renet::ServerEvent::{ClientConnected, ClientDisconnected};

use crate::client::resources::{
    ClientId, ClientLobby, CurrentClientId, NetworkEntities, PlayerInfo,
};
use crate::components::{
    Animator, Controllable, EquipmentBundle, Player, PlayerBundle, PlayerCamera,
    ServerEquipmentBundle, ServerPlayerBundle, Team,
};
use crate::enums::{CollisionGroups, Equipment, Sprites};
use crate::events::{
    ClientConnectedEvent, ClientDisconnectedEvent, CreatePlayerEvent, RemovePlayerEvent,
};
use crate::networking::networking::ServerMessages;
use crate::networking::ServerChannel;
use crate::resources::{AssetHandler, AssetsConfig, ServerLobby};

pub fn create_player(
    mut commands: Commands,
    mut reader_player_create: EventReader<CreatePlayerEvent>,
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

        let character_type = Sprites::Skeleton;
        // Retrieve character assets from the already loaded resources
        let (texture, animations, hitbox_config) = asset_handler
            .textures
            .get(&character_type)
            .expect("unexpected character requested.");

        let hitbox_config = hitbox_config.expect(&format!(
            "Requested character does not have a hitbox: {:?}",
            character_type
        ));

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
            Vec2::new(hitbox_config.width, hitbox_config.height),
            CollisionGroup {
                layer: CollisionGroups::Player as u32 | player_create_event.team,
                mask: 0,
            },
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
        let (texture, animations, _hitbox_config) = asset_handler
            .textures
            .get(&Sprites::AK47)
            .expect("unexpected character requested.");

        // Spawn Equipment
        commands
            .spawn(EquipmentBundle::new(
                asset_config
                    .stats
                    .equipment
                    .get(&Equipment::AK47)
                    .expect("Could not find AK47 in equipment config.")
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

pub fn player_despawn(
    mut commands: Commands,
    mut reader_player_remove: EventReader<RemovePlayerEvent>,
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
