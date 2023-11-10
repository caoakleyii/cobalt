use bevy_renet::renet::ConnectionConfig;

use super::{ClientChannel, ServerChannel};

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}

pub const PROTOCOL_ID: u64 = 7;
