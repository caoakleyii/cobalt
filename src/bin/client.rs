use bevy::prelude::*;
use bevy_2d_collisions::CollisionsPlugin;
use bevy_health_bar::ProgressBarPlugin;
use bevy_renet::{transport::NetcodeClientPlugin, RenetClientPlugin};

use utils::client::sets::Connected;
use utils::client::ClientPlugin;
use utils::events::DamageEntityEvent;
use utils::systems::on_damage_entity;
use utils::{
    enums::GameState,
    events::{
        CreatePlayerEvent, EquippedUse, PlayerCommand, RemovePlayerEvent, SpawnProjectileEvent,
    },
    resources::{AssetLoading, PlayerInput, TextAsset, TextLoader},
    systems::{
        animate_sprites, apply_direction, apply_velocity, asset_config_loader_sytem,
        asset_loader_state_system, asset_loader_system, capture_player_command_input_system,
        capture_player_input_system, client_send_player_command_events,
        client_send_player_input_system, create_player, handle_input, health_bar_update,
        player_despawn, spawn_projectile, sync_animation_state, tick_equipment_system,
    },
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        RenetClientPlugin,
        NetcodeClientPlugin,
        ClientPlugin,
        ProgressBarPlugin,
        CollisionsPlugin,
    ));

    app.add_state::<GameState>();

    register_client_asset_systems(&mut app);

    register_network_events(&mut app);

    reigster_game_systems(&mut app);

    app.run();
}

fn register_client_asset_systems(app: &mut App) {
    app.init_asset::<TextAsset>();
    app.init_asset_loader::<TextLoader>();

    app.add_systems(Startup, asset_config_loader_sytem);
    app.add_systems(
        Update,
        asset_loader_system.run_if(in_state(GameState::Loading)),
    );

    app.add_systems(Update, (animate_sprites, asset_loader_state_system));
}
/// Network Systems and Events once the client is connected
fn register_network_events(app: &mut App) {
    app.add_event::<CreatePlayerEvent>();
    app.add_event::<RemovePlayerEvent>();
    app.add_event::<SpawnProjectileEvent>();
    app.add_event::<DamageEntityEvent>();

    app.add_systems(
        Update,
        (
            create_player,
            player_despawn,
            client_send_player_command_events,
        )
            .in_set(Connected),
    );
}

/// Game Loop Systems outside of network
fn reigster_game_systems(app: &mut App) {
    app.insert_resource(PlayerInput::default());
    app.insert_resource(AssetLoading::default());

    // Some of this can be moved to a plugin
    // or abstracted so both client and server can use it
    app.add_event::<EquippedUse>();
    app.add_event::<PlayerCommand>();

    app.add_systems(
        Update,
        (
            capture_player_input_system,
            capture_player_command_input_system,
            client_send_player_input_system,
            handle_input,
            sync_animation_state,
            tick_equipment_system,
            apply_velocity,
            spawn_projectile,
            apply_direction,
            on_damage_entity,
            health_bar_update,
        )
            .in_set(Connected),
    );
}
