use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    enums::GameState,
    networking::{is_client, is_server},
};

use self::systems::*;

pub mod components;
pub mod resources;
mod systems;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                capture_player_input_system,
                capture_player_command_input_system,
                client_send_player_input_system,
                client_send_player_command_events,
                handle_movement_input,
                handle_deck_input,
            )
                .run_if(in_state(GameState::Gameloop))
                .run_if(is_client()),
        );

        app.add_systems(
            Update,
            (
                server_receive_player_input_system,
                handle_movement_input,
                server_receive_player_command_system,
                handle_deck_input,
            )
                .run_if(in_state(GameState::Gameloop))
                .run_if(is_server()),
        );
    }
}
