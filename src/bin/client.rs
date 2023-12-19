use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy_health_bar::ProgressBarPlugin;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
        RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};

use utils::{
    enums::GameState,
    events::{
        EquippedUse, PlayerCommand, PlayerCreateEvent, PlayerRemoveEvent, SpawnProjectileEvent,
    },
    networking::{connection_config, PROTOCOL_ID},
    resources::{
        AssetLoading, ClientLobby, Connected, CurrentClientId, NetworkEntities, PlayerInput,
        TextAsset, TextLoader,
    },
    systems::{
        animate_sprites, apply_direction, apply_velocity, asset_config_loader_sytem,
        asset_loader_state_system, asset_loader_system, capture_player_command_input_system,
        capture_player_input_system, client_send_player_command_events,
        client_send_player_input_system, handle_input, networking::client_update_system,
        player_create_system, player_remove_system, spawn_projectile, sync_animation_state,
        tick_equipment_system,
    },
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        RenetClientPlugin,
        NetcodeClientPlugin,
        ProgressBarPlugin,
    ));

    app.add_state::<GameState>();

    register_client_asset_systems(&mut app);

    connect_client_and_network_systems(&mut app);

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

/// Connnect to the server
/// and add any required resources, and systems.
fn connect_client_and_network_systems(app: &mut App) {
    let client = RenetClient::new(connection_config());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.configure_sets(Update, Connected.run_if(client_connected()));
    app.insert_resource(client);
    app.insert_resource(transport);

    app.insert_resource(ClientLobby::default());
    app.insert_resource(CurrentClientId(client_id));
    app.insert_resource(NetworkEntities::default());

    // If any error is found we just panic
    fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
        for e in renet_error.read() {
            panic!("{}", e);
        }
    }

    app.add_systems(
        Update,
        panic_on_error_system.run_if(in_state(GameState::Gameloop)),
    );
    app.add_systems(
        Update,
        client_update_system.run_if(in_state(GameState::Gameloop)),
    );
}

/// Network Systems and Events once the client is connected
fn register_network_events(app: &mut App) {
    app.add_event::<PlayerCreateEvent>();
    app.add_event::<PlayerRemoveEvent>();
    app.add_event::<SpawnProjectileEvent>();

    app.add_systems(
        Update,
        (
            player_create_system,
            player_remove_system,
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
        )
            .in_set(Connected),
    );
}
