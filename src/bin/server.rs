use std::time::Duration;
use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::RenetServer;
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use utils::events::{ClientConnectedEvent, ClientDisconnectedEvent, PlayerInputEvent};
use utils::networking::{connection_config, PROTOCOL_ID};
use utils::resources::ServerLobby;
use utils::systems::networking::server::server_update_system;
use utils::systems::{
    client_connected_system, client_disconnected_system, handle_input, server_network_sync,
    server_receive_player_input_system,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, RenetServerPlugin, NetcodeServerPlugin));

    build_server_and_network_systems(&mut app);

    register_network_events(&mut app);

    app.run();
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

    app.add_systems(Update, server_update_system);
}

fn register_network_events(app: &mut App) {
    app.add_event::<ClientConnectedEvent>();
    app.add_event::<ClientDisconnectedEvent>();
    app.add_event::<PlayerInputEvent>();

    app.add_systems(
        Update,
        (
            client_connected_system,
            client_disconnected_system,
            server_receive_player_input_system,
            server_network_sync,
            handle_input,
        ),
    );
}
