use bevy::prelude::*;
use bevy_2d_collisions::CollisionsPlugin;

use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::{
    deck::DeckPlugin,
    enums::GameState,
    input::InputPlugin,
    physics::PhysicsPlugin,
    server::ServerPlugin,
    stats::StatsPlugin,
    systems::{
        apply_velocity, handle_input, server_receive_player_command_system,
        server_receive_player_input_system,
    },
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        PhysicsPlugin,
        RenetServerPlugin,
        NetcodeServerPlugin,
        ServerPlugin,
        CollisionsPlugin,
        InternalAssetPlugin,
        DeckPlugin,
        InputPlugin,
        StatsPlugin,
    ));

    app.add_state::<GameState>();

    app.run();
}
