use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
        RenetClient,
    },
};

use self::resources::*;
use crate::{
    client::sets::Connected,
    enums::GameState,
    networking::config::{connection_config, PROTOCOL_ID},
};

use self::systems::client_update_system;

pub mod resources;
pub mod sets;
mod systems;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        self.connect_client_and_network_systems(app);

        app.add_systems(
            Update,
            client_update_system.run_if(in_state(GameState::Gameloop)),
        );
    }
}

impl ClientPlugin {
    /// Connnect to the server
    /// and add any required resources, and systems.
    fn connect_client_and_network_systems(&self, app: &mut App) {
        // TODO turn this into a system that runs once in connecting game state
        let client = RenetClient::new(connection_config());

        // let server_addr = "127.0.0.1:5000".parse().unwrap();
        let server_addr = "138.197.16.199:5000".parse().unwrap();
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
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
                panic!("{:?}", e);
            }
        }

        app.add_systems(Update, panic_on_error_system.in_set(Connected));
    }
}
