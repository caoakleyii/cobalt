use bevy::prelude::*;
use bevy_2d_collisions::CollisionsPlugin;

use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::asset::AssetPlugin as InternalAssetPlugin;
use utils::deck::DeckPlugin;
use utils::enums::GameState;

use utils::server::ServerPlugin;
use utils::systems::{
    apply_velocity, handle_input, server_receive_player_command_system,
    server_receive_player_input_system,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        RenetServerPlugin,
        NetcodeServerPlugin,
        ServerPlugin,
        CollisionsPlugin,
        InternalAssetPlugin,
        DeckPlugin,
    ));

    app.add_state::<GameState>();

    register_network_events(&mut app);

    app.run();
}

fn register_network_events(app: &mut App) {
    app.add_systems(
        Update,
        (
            server_receive_player_input_system,
            handle_input,
            server_receive_player_command_system,
            apply_velocity,
        )
            .run_if(in_state(GameState::Gameloop)),
    );
}
