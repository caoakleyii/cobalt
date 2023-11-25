use bevy::prelude::{Commands, EventWriter, ResMut, Transform};
use bevy_renet::renet::RenetClient;

use crate::{
    enums::ServerMessages,
    events::{PlayerCreateEvent, PlayerRemoveEvent, SpawnProjectileEvent},
    networking::{NetworkedEntities, ServerChannel},
    resources::NetworkEntities,
};

pub fn client_update_system(
    mut writer_player_create: EventWriter<PlayerCreateEvent>,
    mut writer_player_remove: EventWriter<PlayerRemoveEvent>,
    mut writer_spawn_projectile: EventWriter<SpawnProjectileEvent>,
    mut client: ResMut<RenetClient>,
    network_mapping: ResMut<NetworkEntities>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message: ServerMessages = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate(player_create_event) => {
                writer_player_create.send(player_create_event);
            }
            ServerMessages::PlayerRemove(player_remove_event) => {
                writer_player_remove.send(player_remove_event);
            }
            ServerMessages::SpawnProjectile(spawn_projectile_event) => {
                writer_spawn_projectile.send(spawn_projectile_event);
            }
        };
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
                if let Some(mut entity_command) = commands.get_entity(*entity) {
                    entity_command.insert((transform, state));
                }
            }
        }
    }
}
