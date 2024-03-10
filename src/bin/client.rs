use bevy::DefaultPlugins;
use bevy::{app::App, winit::WinitSettings};
use bevy_2d_collisions::CollisionsPlugin;
use bevy_health_bar::ProgressBarPlugin;
use bevy_renet::{transport::NetcodeClientPlugin, RenetClientPlugin};

use utils::{
    animation::AnimationPlugin, asset::AssetPlugin as InternalAssetPlugin, client::ClientPlugin,
    deck::DeckPlugin, enums::GameState, input::InputPlugin, physics::PhysicsPlugin,
    player::PlayerPlugin, stats::StatsPlugin, ui::UiPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        RenetClientPlugin,
        NetcodeClientPlugin,
        ClientPlugin,
        InternalAssetPlugin,
        InputPlugin,
        AnimationPlugin,
        PhysicsPlugin,
        StatsPlugin,
        ProgressBarPlugin,
        CollisionsPlugin,
        PlayerPlugin,
        DeckPlugin,
        UiPlugin,
    ));

    app.add_state::<GameState>();

    app.run();
}
