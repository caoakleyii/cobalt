use bevy::prelude::*;

use crate::{client::sets::Connected, enums::GameState, input::resources::PlayerInput};

use self::{
    events::{CreatePlayerEvent, PlayerCommand, RemovePlayerEvent},
    systems::{create_player, player_despawn},
};

pub mod components;
pub mod events;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (create_player, player_despawn)
                .run_if(in_state(GameState::Gameloop))
                .in_set(Connected),
        );

        app.add_event::<CreatePlayerEvent>();
        app.add_event::<RemovePlayerEvent>();
        app.add_event::<PlayerCommand>();

        app.insert_resource(PlayerInput::default());
    }
}
