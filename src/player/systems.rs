use super::components::{LocalPlayer, Player, Team};
use super::events::{EntitySpawnedEvent, SpawnPlayerEvent};
use crate::animation::events::SpawnSpriteEvent;
use crate::asset::enums::Sprites;
use crate::asset::resources::AssetHandler;

use crate::client::resources::{
    ClientId, ClientLobby, CurrentClientId, NetworkEntities, PlayerInfo,
};
use crate::deck::card::components::{Card, CardEntity};
use crate::deck::card::enums::Cards;
use crate::deck::components::DeckBundle;
use crate::enums::CollisionGroups;
use crate::input::components::{Controllable, FollowCamera, PlayerCamera};
use crate::networking::channels::ServerChannel;
use crate::networking::networking::ServerMessages;
use crate::player::components::PlayerBundle;
use crate::player::events::{CreatePlayerEvent, RemovePlayerEvent};
use crate::server::resources::ServerLobby;
use bevy::prelude::*;
use bevy_2d_collisions::components::CollisionGroup;
use bevy_renet::renet::{ClientId as RenetClientId, RenetServer};

pub fn create_player(
    mut commands: Commands,
    mut reader_player_create: EventReader<CreatePlayerEvent>,
    mut write_spawn_player: EventWriter<SpawnPlayerEvent>,
    mut write_spawn_sprite: EventWriter<SpawnSpriteEvent>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkEntities>,
    mut windows: Query<&mut Window>,
    client_id: Res<CurrentClientId>,
) {
    for player_create_event in reader_player_create.read() {
        println!("Player {} connected.", player_create_event.id.0);

        let is_local_player = player_create_event.id.0 == client_id.0;

        if is_local_player {
            let mut camera_bundle = Camera2dBundle::default();
            camera_bundle.projection.scale = 0.5;
            commands.spawn((camera_bundle, PlayerCamera));
        }

        let mut player_entity_command = commands.spawn_empty();
        let player_entity = player_entity_command.id();

        if is_local_player {
            player_entity_command.insert(Controllable);
            let mut window = windows.single_mut();
            window.cursor.icon = bevy::window::CursorIcon::Crosshair;
        }

        let player_info = PlayerInfo {
            server_entity: player_create_event.entity,
            client_entity: player_entity,
        };

        lobby.players.insert(player_create_event.id, player_info);
        network_mapping
            .0
            .insert(player_create_event.entity, player_entity);

        write_spawn_player.send(SpawnPlayerEvent {
            entity: player_entity,
            id: player_create_event.id,
            translation: player_create_event.translation,
            team: player_create_event.team,
            local_player: is_local_player,
        });

        write_spawn_sprite.send(SpawnSpriteEvent {
            entity: Some(player_entity),
            sprite: Sprites::Skeleton,
            translation: Some(Vec3::new(
                player_create_event.translation[0],
                player_create_event.translation[1],
                player_create_event.translation[2],
            )),
            ..Default::default()
        });

        // Retrieve character assets from the already loaded resources
        // let (_texture, _animations, _hitbox_config) = asset_handler
        //     .textures
        //     .get(&Sprites::AK47)
        //     .expect("unexpected character requested.");
        // Spawn Equipment
        // commands
        //     .spawn(EquipmentBundle::new(
        //         asset_config
        //             .stats
        //             .equipment
        //             .get(&Equipment::AK47)
        //             .expect("Could not find AK47 in equipment config.")
        //             .into(),
        //         Animator::import(animations),
        //         texture_atlases.add(texture.clone()),
        //         Transform::from_xyz(5.0, -1.5, 0.0),
        //     ))
        //     .set_parent(player_entity);
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

pub fn create_player_server(
    mut reader_create_player: EventReader<CreatePlayerEvent>,
    mut writer_spawn_player: EventWriter<SpawnPlayerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &Player, &Transform, &Team)>,
) {
    for create_player_event in reader_create_player.read() {
        let client_id = RenetClientId::from_raw(create_player_event.id.0);
        println!("Player {} connected.", client_id);

        // initialize the newly connected client with the current state of the players in the game
        for (entity, player, transform, p_team) in &players {
            let translation: [f32; 3] = transform.translation.into();
            let message = bincode::serialize(&ServerMessages::PlayerCreate(CreatePlayerEvent {
                id: ClientId(player.id.raw()),
                entity,
                translation,
                team: (**p_team).into(),
            }))
            .unwrap();
            server.send_message(client_id, ServerChannel::ServerMessages, message);
        }

        // TODO: Change this, curently just auto balancing teams
        let team = if lobby.players.len() % 2 == 0 {
            CollisionGroups::TeamAlpha as u32
        } else {
            CollisionGroups::TeamBravo as u32
        };

        let spawn_point = Vec3::new(0.0, 0.0, 0.0);
        let player_entity = commands.spawn_empty().id();

        lobby.players.insert(client_id.raw(), player_entity);

        // send the player entity to the clients
        let message = bincode::serialize(&ServerMessages::PlayerCreate(CreatePlayerEvent {
            id: ClientId(client_id.raw()),
            entity: player_entity,
            translation: spawn_point.to_array(),
            team,
        }))
        .unwrap();
        server.broadcast_message(ServerChannel::ServerMessages, message);

        // spawn the player on the server
        writer_spawn_player.send(SpawnPlayerEvent {
            entity: player_entity,
            id: ClientId(client_id.raw()),
            translation: spawn_point.to_array(),
            team,
            local_player: false,
        });
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut reader_spawn_player: EventReader<SpawnPlayerEvent>,
    mut writer_entity_spawned: EventWriter<EntitySpawnedEvent>,
    asset_handler: Res<AssetHandler>,
) {
    for spawn_player_event in reader_spawn_player.read() {
        let character_type = Sprites::Skeleton;

        let texture = asset_handler
            .textures
            .get(&character_type)
            .expect("unexpected character requested.");

        let hitbox_config = texture.hitbox.expect(&format!(
            "Requested character does not have a hitbox: {:?}",
            character_type
        ));

        let player_bundle = PlayerBundle::new(
            RenetClientId::from_raw(spawn_player_event.id.0),
            Transform::from_xyz(
                spawn_player_event.translation[0],
                spawn_player_event.translation[1],
                spawn_player_event.translation[2],
            ),
            Vec2::new(hitbox_config.width, hitbox_config.height),
            CollisionGroup {
                layer: CollisionGroups::Player as u32 | spawn_player_event.team,
                mask: 0,
            },
            Team(spawn_player_event.team.into()),
        );

        let default_deck = vec![
            Cards::AK47,
            Cards::AK47,
            Cards::AK47,
            Cards::FragGrenade,
            Cards::FragGrenade,
            Cards::SupplyDrop,
            Cards::Metal,
            Cards::Metal,
        ];

        let cards: Vec<CardEntity> = default_deck
            .iter()
            .map(|card| {
                let card = asset_handler
                    .cards
                    .get(card)
                    .expect("unexpected card requested")
                    .clone();
                let entity = commands.spawn(card.clone()).id();

                CardEntity { entity, card }
            })
            .collect();

        let deck_bundle = DeckBundle::new(cards);
        let mut entity_commands = commands.get_or_spawn(spawn_player_event.entity);

        entity_commands.insert(player_bundle).insert(deck_bundle);

        println!(
            "Player spawned: {:?} with deck bundle",
            spawn_player_event.entity
        );

        if spawn_player_event.local_player {
            entity_commands.insert((LocalPlayer, FollowCamera));
        }

        writer_entity_spawned.send(EntitySpawnedEvent {
            entity: spawn_player_event.entity,
        });
    }
}

pub fn camera_follow_player(
    players: Query<&Transform, With<FollowCamera>>,
    mut query_camera: Query<&mut Transform, (With<PlayerCamera>, Without<FollowCamera>)>,
) {
    if let Ok(player) = players.get_single() {
        if let Ok(mut camera_transform) = query_camera.get_single_mut() {
            camera_transform.translation = player.translation;
        }
    }
}
