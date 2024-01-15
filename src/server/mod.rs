use std::{
    net::UdpSocket,
    time::{Duration, SystemTime},
};

use bevy::prelude::*;
use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    RenetServer,
};

use crate::{
    enums::GameState,
    networking::config::{connection_config, PROTOCOL_ID},
};

use self::{
    events::{
        ClientConnectedEvent, ClientDisconnectedEvent, ClientSentCommandEvent, ClientSentInputEvent,
    },
    resources::ServerLobby,
    systems::{
        client_connected_to_server, client_disconnected, server_network_sync, server_update_system,
    },
};

pub mod events;
pub mod resources;
mod systems;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        host_server(app);

        app.add_systems(
            Update,
            (
                client_connected_to_server,
                client_disconnected,
                server_network_sync,
                server_update_system,
            )
                .run_if(in_state(GameState::Gameloop)),
        );

        app.add_event::<ClientConnectedEvent>();
        app.add_event::<ClientDisconnectedEvent>();
        app.add_event::<ClientSentInputEvent>();
        app.add_event::<ClientSentCommandEvent>();

        app.insert_resource(ServerLobby::default());
    }
}

fn host_server(app: &mut App) {
    let server = RenetServer::new(connection_config());

    let public_addr = "0.0.0.0:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        current_time,
        public_addresses: vec![std::net::SocketAddr::V4(public_addr)],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(server);
    app.insert_resource(transport);
}
