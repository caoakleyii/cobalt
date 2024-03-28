use bevy::prelude::*;

use crate::{
    client::sets::{ClientConnected, PlayerSpawnSet},
    enums::GameState,
    input::resources::PlayerInput,
    networking::{is_client, is_server},
};

use self::{
    events::{
        CreatePlayerEvent, EntitySpawnedEvent, PlayerCommand, RemovePlayerEvent, SpawnPlayerEvent,
    },
    systems::{
        camera_follow_player, create_player, create_player_server, player_despawn, spawn_player,
    },
};

pub mod components;
pub mod events;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (create_player, player_despawn, camera_follow_player)
                .run_if(in_state(GameState::Gameloop))
                .run_if(is_client())
                .in_set(ClientConnected),
        );

        app.add_systems(
            Update,
            (create_player_server)
                .run_if(in_state(GameState::Gameloop))
                .run_if(is_server()),
        );

        app.add_systems(
            Update,
            spawn_player
                .run_if(in_state(GameState::Gameloop))
                .in_set(PlayerSpawnSet),
        );

        app.add_event::<CreatePlayerEvent>();
        app.add_event::<SpawnPlayerEvent>();
        app.add_event::<EntitySpawnedEvent>();
        app.add_event::<RemovePlayerEvent>();
        app.add_event::<PlayerCommand>();

        app.insert_resource(PlayerInput::default());
    }
}
