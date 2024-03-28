use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, Res},
    },
    prelude::{Commands, EventWriter, ResMut, Transform},
    time::{Time, Timer, TimerMode},
};
use bevy_renet::renet::RenetClient;

use crate::{
    animation::events::PlayAnimationEvent,
    deck::{
        events::ShuffleEvent,
        keyword::events::{DamageEntityEvent, SpawnProjectileEvent},
    },
    enums::EntityState::Dead,
    input::components::{Aim, Controllable},
    networking::{
        channels::ServerChannel,
        models::NetworkedEntities,
        networking::{ReplayedServerMessage, ServerMessage, ServerMessages},
    },
    player::{
        components::Death,
        events::{CreatePlayerEvent, RemovePlayerEvent},
    },
};

use super::{resources::NetworkEntities, ReplayMessageExpiry};

pub fn client_update_system(
    mut client: ResMut<RenetClient>,
    network_mapping: ResMut<NetworkEntities>,
    mut writer_server_message: EventWriter<ServerMessage>,
    mut writer_play_animation: EventWriter<PlayAnimationEvent>,
    mut commands: Commands,
    query: Query<Option<&Controllable>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize::<ServerMessages>(&message);
        if server_message.is_err() {
            println!(
                "Failed to deserialize server message {:?}",
                server_message.unwrap_err()
            );
            continue;
        }
        let server_message = server_message.unwrap();
        writer_server_message.send(ServerMessage::new(server_message));
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        // TODO: Possibly worth breaking out into seperate event reader stream
        for i in 0..networked_entities.entities.len() {
            // If we don't have the synced entity, currently just skip it
            // TODO: Consider a factory implementation similar to Litihum here
            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation = networked_entities.translations[i].into();
                let transform = Transform {
                    translation,
                    ..Default::default()
                };
                let state = networked_entities.states[i];
                let aim = Aim(networked_entities.aim_ats[i].into());
                if let Some(mut entity_command) = commands.get_entity(*entity) {
                    if let Ok(controllable) = query.get(*entity) {
                        if let Some(_) = controllable {
                            // if local player is controlling
                            // we should only lerp the position, if it's very inaccurate
                            // and we don't need to update the aim.
                            entity_command.insert((state));
                        } else {
                            // TODO: Lerp transform
                            entity_command.insert((transform, state, aim));
                        }

                        if state == Dead {
                            entity_command.insert(Death::default());
                        }
                    }
                }

                writer_play_animation.send(PlayAnimationEvent::new(*entity, &state.to_string()));
            }
        }
    }
}

pub fn handle_server_messages(
    mut reader_server_messages: EventReader<ServerMessage>,
    mut writer_player_create: EventWriter<CreatePlayerEvent>,
    mut writer_player_remove: EventWriter<RemovePlayerEvent>,
    mut writer_spawn_projectile: EventWriter<SpawnProjectileEvent>,
    mut writer_damage_entity: EventWriter<DamageEntityEvent>,
    mut writer_shuffle_event: EventWriter<ShuffleEvent>,
    mut writer_replayable_server_messages: EventWriter<ReplayedServerMessage>,
    network_mapping: ResMut<NetworkEntities>,
) {
    for server_message in reader_server_messages.read() {
        match server_message.message.clone() {
            ServerMessages::PlayerCreate(player_create_event) => {
                writer_player_create.send(player_create_event);
            }
            ServerMessages::PlayerRemove(player_remove_event) => {
                writer_player_remove.send(player_remove_event);
            }
            ServerMessages::SpawnProjectile(spawn_projectile_event) => {
                writer_spawn_projectile.send(spawn_projectile_event);
            }
            ServerMessages::DamageEntity(damage_entity_event) => {
                writer_damage_entity.send(damage_entity_event);
            }
            ServerMessages::Shuffle(shuffle_event) => {
                println!("Shuffle received for entity {:?}", shuffle_event.player);
                if let Some(entity) = network_mapping.0.get(&shuffle_event.player) {
                    println!("Entity mapped correctly to {:?}", entity);
                    writer_shuffle_event.send(ShuffleEvent {
                        player: *entity,
                        seed: shuffle_event.seed,
                    });
                } else {
                    println!("Mapping miss, replaying  message");
                    writer_replayable_server_messages
                        .send(ReplayedServerMessage::new(server_message.clone()));
                }
            }
        };
    }
}

pub fn replay_server_message(
    mut writer_server_message: EventWriter<ServerMessage>,
    mut reader_replayed_server_messages: EventReader<ReplayedServerMessage>,
    mut expiry: ResMut<ReplayMessageExpiry>,
    dt: Res<Time>,
) {
    for replayed_server_message in reader_replayed_server_messages.read() {
        if let Some(duration) = expiry.messages.get_mut(&replayed_server_message.message_id) {
            duration.tick(dt.delta());
            if duration.finished() {
                expiry.messages.remove(&replayed_server_message.message_id);
                return;
            }
        } else {
            expiry.messages.insert(
                replayed_server_message.message_id,
                Timer::from_seconds(10.0, TimerMode::Once),
            );
        }
        writer_server_message.send(ServerMessage::new(replayed_server_message.message.clone()));
    }
}
