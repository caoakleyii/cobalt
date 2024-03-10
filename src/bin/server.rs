use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_2d_collisions::CollisionsPlugin;

use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::{
    animation::AnimationPlugin, asset::AssetPlugin as InternalAssetPlugin, deck::DeckPlugin,
    enums::GameState, input::InputPlugin, physics::PhysicsPlugin, player::PlayerPlugin,
    server::ServerPlugin, stats::StatsPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
        AssetPlugin::default(),
        PhysicsPlugin,
        RenetServerPlugin,
        NetcodeServerPlugin,
        ServerPlugin,
        CollisionsPlugin,
        AnimationPlugin,
        InternalAssetPlugin,
        PlayerPlugin,
        DeckPlugin,
        InputPlugin,
        StatsPlugin,
    ));

    app.add_state::<GameState>();

    app.run();
}
