use bevy::prelude::*;

use crate::{client::sets::Connected, enums::GameState};

use self::{
    events::{CreatePlayerEvent, PlayerCommand, RemovePlayerEvent},
    systems::{client_send_player_command_events, create_player, player_despawn},
};

pub mod events;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                create_player,
                player_despawn,
                client_send_player_command_events,
            )
                .run_if(in_state(GameState::Gameloop))
                .in_set(Connected),
        );

        app.add_event::<CreatePlayerEvent>();
        app.add_event::<RemovePlayerEvent>();
        app.add_event::<PlayerCommand>();
    }
}
