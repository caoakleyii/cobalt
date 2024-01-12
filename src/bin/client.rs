use bevy::prelude::*;
use bevy_2d_collisions::CollisionsPlugin;
use bevy_health_bar::ProgressBarPlugin;
use bevy_renet::{transport::NetcodeClientPlugin, RenetClientPlugin};

use utils::asset::AssetPlugin as InternalAssetPlugin;
use utils::client::sets::Connected;
use utils::client::ClientPlugin;
use utils::deck::DeckPlugin;
use utils::player::PlayerPlugin;
use utils::resources::PlayerInput;
use utils::{
    enums::GameState,
    systems::{
        animate_sprites, apply_direction, apply_velocity, capture_player_command_input_system,
        capture_player_input_system, client_send_player_input_system, handle_input,
        health_bar_update, sync_animation_state,
    },
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        RenetClientPlugin,
        NetcodeClientPlugin,
        ClientPlugin,
        InternalAssetPlugin,
        ProgressBarPlugin,
        CollisionsPlugin,
        PlayerPlugin,
        DeckPlugin,
    ));

    app.add_state::<GameState>();

    register_client_asset_systems(&mut app);

    reigster_game_systems(&mut app);

    app.run();
}

fn register_client_asset_systems(app: &mut App) {
    app.add_systems(Update, (animate_sprites));
}

/// Game Loop Systems outside of network
fn reigster_game_systems(app: &mut App) {
    app.insert_resource(PlayerInput::default());

    app.add_systems(
        Update,
        (
            capture_player_input_system,
            capture_player_command_input_system,
            client_send_player_input_system,
            handle_input,
            sync_animation_state,
            apply_velocity,
            apply_direction,
            health_bar_update,
        )
            .in_set(Connected),
    );
}
