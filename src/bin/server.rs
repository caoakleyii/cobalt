use bevy::prelude::*;
use bevy_2d_collisions::CollisionsPlugin;

use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::enums::GameState;
use utils::events::{EquippedUse, SpawnProjectileEvent};

use utils::resources::{TextAsset, TextLoader};
use utils::server::ServerPlugin;
use utils::systems::{
    apply_velocity, asset_config_loader_sytem, asset_loader_system, damage_collision,
    equipment_use_system, handle_input, server_receive_player_command_system,
    server_receive_player_input_system, tick_equipment_system,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        RenetServerPlugin,
        NetcodeServerPlugin,
        CollisionsPlugin,
        ServerPlugin,
    ));

    app.add_state::<GameState>();

    register_server_asset_systems(&mut app);

    register_network_events(&mut app);

    app.run();
}

fn register_server_asset_systems(app: &mut App) {
    app.init_asset::<TextAsset>();
    app.init_asset::<Image>();
    app.init_asset_loader::<TextLoader>();
    app.add_systems(Startup, asset_config_loader_sytem);
    app.add_systems(
        Update,
        asset_loader_system.run_if(in_state(GameState::Loading)),
    );
}

fn register_network_events(app: &mut App) {
    app.add_event::<EquippedUse>();
    app.add_event::<SpawnProjectileEvent>();

    app.add_systems(
        Update,
        (
            server_receive_player_input_system,
            handle_input,
            server_receive_player_command_system,
            tick_equipment_system,
            equipment_use_system,
            apply_velocity,
            damage_collision,
        )
            .run_if(in_state(GameState::Gameloop)),
    );
}
