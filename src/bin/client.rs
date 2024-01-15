use bevy::app::{App, Update};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::DefaultPlugins;
use bevy_2d_collisions::CollisionsPlugin;
use bevy_health_bar::ProgressBarPlugin;
use bevy_renet::{transport::NetcodeClientPlugin, RenetClientPlugin};

use utils::animation::AnimationPlugin;
use utils::asset::AssetPlugin as InternalAssetPlugin;
use utils::client::sets::Connected;
use utils::client::ClientPlugin;
use utils::deck::DeckPlugin;
use utils::player::PlayerPlugin;
use utils::resources::PlayerInput;
use utils::{
    enums::GameState,
    systems::{
        apply_direction, apply_velocity, capture_player_command_input_system,
        capture_player_input_system, client_send_player_input_system, handle_input,
        health_bar_update,
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
        AnimationPlugin,
        ProgressBarPlugin,
        CollisionsPlugin,
        PlayerPlugin,
        DeckPlugin,
    ));

    app.add_state::<GameState>();

    reigster_game_systems(&mut app);

    app.run();
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
            apply_velocity,
            apply_direction,
            health_bar_update,
        )
            .in_set(Connected),
    );
}
