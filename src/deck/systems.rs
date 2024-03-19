use std::time::{SystemTime, UNIX_EPOCH};

use bevy::ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, Query, ResMut},
};
use bevy_renet::renet::RenetServer;

use crate::{
    deck::components::Shuffled,
    networking::{channels::ServerChannel, networking::ServerMessages},
    player::events::PlayerSpawnedEvent,
};

use super::{components::Library, events::ShuffleEvent};

use rand::{rngs::SmallRng, seq::SliceRandom};

// Server Only
// player_spawned_write_to_shuffle
// sends rng seed
pub fn player_spawned_spawn_deck(
    mut reader_player_spawned: EventReader<PlayerSpawnedEvent>,
    mut writer_shuffle: EventWriter<ShuffleEvent>,
    mut server: ResMut<RenetServer>,
) {
    for player in reader_player_spawned.read() {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let shuffle_event = ShuffleEvent {
            entity: player.entity,
            seed,
        };

        server.broadcast_message(
            ServerChannel::ServerMessages,
            bincode::serialize(&ServerMessages::Shuffle(shuffle_event.clone()))
                .expect("Could not serialize shuffle message."),
        );
        writer_shuffle.send(shuffle_event);
    }
}

// Server and Client
// read from shuffle event
// use rng seed to shuffle the deck
pub fn shuffle_deck(
    mut events: EventReader<ShuffleEvent>,
    mut library_query: Query<&mut Library>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok(mut library) = library_query.get_mut(event.entity) {
            println!("Shuffling deck for entity: {:?}", event.entity);
            let mut rng = SmallRng::seed_from_u64(event.seed);
            library.0.shuffle(&mut rng);

            commands.entity(event.entity).insert(Shuffled);
        }
    }
}
