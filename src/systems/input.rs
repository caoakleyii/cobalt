use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_renet::renet::RenetClient;

use crate::components::{Controllable, Equipped, Player, PlayerCamera, Velocity};

use crate::enums::EntityState;
use crate::events::{EquippedUse, PlayerCommand, PlayerCommandEvent, PlayerInputEvent};
use crate::networking::ClientChannel;
use crate::resources::{ClientId, ClientLobby, CurrentClientId, PlayerInput, ServerLobby};

pub fn capture_player_input_system(
    mut player_input: ResMut<PlayerInput>,
    lobby: ResMut<ClientLobby>,
    mut command: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    client_id: Res<CurrentClientId>,
) {
    // get the camera info and transform
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            player_input.aim = world_position;
        }
    }

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
    // This is not optimized. We should only send the inputs that are true. (Should be serailized as inputs: Vec<Input>, aim: [f32; 2])
    let input_message = bincode::serialize(&*player_input).unwrap();

    client.send_message(ClientChannel::Input, input_message);
}

pub fn capture_player_command_input_system(
    mouse_input: Res<Input<MouseButton>>,
    player_input: Res<PlayerInput>,
    _keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Children, (With<Controllable>, With<Player>)>,
    equipment_query: Query<Entity, With<Equipped>>,
    mut writer_player_command_event: EventWriter<PlayerCommand>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        if let Ok(children) = player_query.get_single() {
            for &child in children.iter() {
                if let Ok(_equipment_entity) = equipment_query.get(child) {
                    writer_player_command_event.send(PlayerCommand::UseEquipment {
                        cast_at: player_input.aim,
                    })
                }
            }
        }
    }
}

// TODO: Probably needs some sort of throttilng to prevent spamming
pub fn client_send_player_command_events(
    mut client: ResMut<RenetClient>,
    mut reader_player_command_event: EventReader<PlayerCommand>,
) {
    for player_command_event in reader_player_command_event.iter() {
        let player_command_message = bincode::serialize(&player_command_event).unwrap();
        client.send_message(ClientChannel::Command, player_command_message);
    }
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

pub fn server_receive_player_command_system(
    mut writer_equippable_use: EventWriter<EquippedUse>,
    mut reader_player_command_event: EventReader<PlayerCommandEvent>,
    lobby: ResMut<ServerLobby>,
) {
    for player_command_event in reader_player_command_event.iter() {
        let player_command = &player_command_event.0;
        let client_id = player_command_event.1;
        if let Some(player_entity) = lobby.players.get(&client_id) {
            match player_command {
                PlayerCommand::UseEquipment { cast_at } => {
                    writer_equippable_use.send(EquippedUse {
                        entity: *player_entity,
                        at: cast_at.clone(),
                    })
                }
            }
        }
    }
}

pub fn handle_input(mut query: Query<(&PlayerInput, &mut Velocity, &mut EntityState)>) {
    for (player_input, mut vel, mut state) in &mut query {
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

        vel.vector.x = force.x * *vel.current_speed;
        vel.vector.y = force.y * *vel.current_speed;
    }
}
