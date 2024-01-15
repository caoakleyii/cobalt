use bevy::prelude::*;
use bevy_2d_collisions::CollisionsPlugin;

use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::{
    asset::AssetPlugin as InternalAssetPlugin, deck::DeckPlugin, enums::GameState,
    input::InputPlugin, physics::PhysicsPlugin, server::ServerPlugin, stats::StatsPlugin,
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
