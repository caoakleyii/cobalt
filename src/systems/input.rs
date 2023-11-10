use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

use crate::components::Velocity;

use crate::enums::EntityState;
use crate::events::PlayerInputEvent;
use crate::networking::ClientChannel;
use crate::resources::{ClientId, ClientLobby, CurrentClientId, PlayerInput, ServerLobby};

pub fn capture_player_input_system(
    mut player_input: ResMut<PlayerInput>,
    lobby: ResMut<ClientLobby>,
    mut command: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    client_id: Res<CurrentClientId>,
) {
    player_input.up = keyboard_input.pressed(KeyCode::W);
    player_input.down = keyboard_input.pressed(KeyCode::S);
    player_input.left = keyboard_input.pressed(KeyCode::A);
    player_input.right = keyboard_input.pressed(KeyCode::D);

    if let Some(current_player_info) = lobby.players.get(&ClientId(client_id.0)) {
        command
            .entity(current_player_info.client_entity)
            .insert(player_input.clone());
    }
}

pub fn client_send_player_input_system(
    player_input: Res<PlayerInput>,
    mut client: ResMut<RenetClient>,
) {
    let input_message = bincode::serialize(&*player_input).unwrap();

    client.send_message(ClientChannel::Input, input_message);
}

pub fn server_receive_player_input_system(
    mut command: Commands,
    mut reader_player_input_event: EventReader<PlayerInputEvent>,
    lobby: ResMut<ServerLobby>,
) {
    for player_input_event in reader_player_input_event.iter() {
        let player_input = player_input_event.0;
        let client_id = player_input_event.1;

        if let Some(player_entity) = lobby.players.get(&client_id) {
            command.entity(*player_entity).insert(player_input);
        }
    }
}

pub fn handle_input(mut query: Query<(&mut Transform, &PlayerInput, &Velocity, &mut EntityState)>) {
    for (mut transform, player_input, vel, mut state) in &mut query {
        let mut fx = 0.0;
        let mut fy = 0.0;

        if player_input.left {
            fx -= 1.0;
        }
        if player_input.right {
            fx += 1.0;
        }
        if player_input.up {
            fy += 1.0;
        }
        if player_input.down {
            fy -= 1.0;
        }
        let force = Vec2::new(fx, fy).normalize_or_zero();

        if force != Vec2::ZERO {
            *state = crate::enums::EntityState::Walk;
        } else {
            *state = crate::enums::EntityState::Idle;
        }

        transform.translation.x += force.x * *vel.current_speed;
        transform.translation.y += force.y * *vel.current_speed;
    }
}
