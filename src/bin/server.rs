use std::time::Duration;
use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::RenetServer;
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::enums::GameState;
use utils::events::{
    ClientConnectedEvent, ClientDisconnectedEvent, EquippedUse, PlayerCommandEvent,
    PlayerInputEvent,
};
use utils::networking::{connection_config, PROTOCOL_ID};
use utils::resources::{ServerLobby, TextAsset, TextLoader};
use utils::systems::networking::server::server_update_system;
use utils::systems::{
    apply_velocity, asset_config_loader_sytem, asset_loader_system,
    client_connected_to_server_system, client_disconnected_system, equipment_use_system,
    handle_input, server_network_sync, server_receive_player_command_system,
    server_receive_player_input_system, tick_equipment_system,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        RenetServerPlugin,
        NetcodeServerPlugin,
    ));

    app.add_state::<GameState>();

    register_server_asset_systems(&mut app);

    build_server_and_network_systems(&mut app);

    register_network_events(&mut app);

    app.run();
}

fn register_server_asset_systems(app: &mut App) {
    app.add_asset::<TextAsset>();
    app.init_asset_loader::<TextLoader>();

    app.add_systems(Startup, asset_config_loader_sytem);
    app.add_systems(
        Update,
        asset_loader_system.run_if(in_state(GameState::Loading)),
    );
}

fn build_server_and_network_systems(app: &mut App) {
    let server = RenetServer::new(connection_config());

    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addr: std::net::SocketAddr::V4(public_addr),
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
    app.insert_resource(server);
    app.insert_resource(transport);
    app.insert_resource(ServerLobby::default());

    app.add_systems(
        Update,
        server_update_system.run_if(in_state(GameState::Gameloop)),
    );
}

fn register_network_events(app: &mut App) {
    app.add_event::<ClientConnectedEvent>();
    app.add_event::<ClientDisconnectedEvent>();
    app.add_event::<PlayerInputEvent>();
    app.add_event::<PlayerCommandEvent>();
    app.add_event::<EquippedUse>();

    app.add_systems(
        Update,
        (
            client_connected_to_server_system,
            client_disconnected_system,
            server_receive_player_input_system,
            server_network_sync,
            handle_input,
            server_receive_player_command_system,
            tick_equipment_system,
            equipment_use_system,
            apply_velocity,
        )
            .run_if(in_state(GameState::Gameloop)),
    );
}
