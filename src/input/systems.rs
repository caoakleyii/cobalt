use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_renet::renet::RenetClient;

use crate::{
    animation::events::PlayAnimationEvent,
    client::resources::{ClientId, ClientLobby, CurrentClientId},
    deck::{
        card::equipment::{components::Equipped, events::EquippedUse},
        components::{Hand, HandSize, Library, Shuffled},
        events::DrawCardEvent,
    },
    enums::EntityState,
    input::resources::PlayerInput,
    networking::channels::ClientChannel,
    physics::components::Velocity,
    player::{
        components::{Death, Player},
        events::PlayerCommand,
    },
    server::{
        events::{ClientSentCommandEvent, ClientSentInputEvent},
        resources::ServerLobby,
    },
};

use super::components::{Aim, Controllable, PlayerCamera};

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

    player_input.draw = keyboard_input.pressed(KeyCode::F);

    if let Some(current_player_info) = lobby.players.get(&ClientId(client_id.0)) {
        command
            .entity(current_player_info.client_entity)
            .insert(player_input.clone())
            .insert(Aim(player_input.aim));
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
    mut writer_player_command_event: EventWriter<PlayerCommand>,
    mut writer_draw_card_event: EventWriter<DrawCardEvent>,
    mouse_input: Res<Input<MouseButton>>,
    player_input: Res<PlayerInput>,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(Entity, &Children), (With<Controllable>, With<Player>)>,
    equipment_query: Query<Entity, With<Equipped>>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        if let Ok((_, children)) = player_query.get_single() {
            for &child in children.iter() {
                if let Ok(_equipment_entity) = equipment_query.get(child) {
                    writer_player_command_event.send(PlayerCommand::UseEquipment {
                        cast_at: player_input.aim,
                    })
                }
            }
        }
    }

    if keyboard_input.pressed(KeyCode::R) {
        if let Ok((entity, _)) = player_query.get_single() {
            writer_draw_card_event.send(DrawCardEvent::new_to_max(entity));
        }
    }
}

// TODO: Probably needs some sort of throttilng to prevent spamming
pub fn client_send_player_command_events(
    mut client: ResMut<RenetClient>,
    mut reader_player_command_event: EventReader<PlayerCommand>,
) {
    for player_command_event in reader_player_command_event.read() {
        let player_command_message = bincode::serialize(&player_command_event).unwrap();
        client.send_message(ClientChannel::Command, player_command_message);
    }
}

// TODO: Rename possibly on_player_input
pub fn server_receive_player_input_system(
    mut command: Commands,
    mut reader_player_input_event: EventReader<ClientSentInputEvent>,
    lobby: ResMut<ServerLobby>,
) {
    for player_input_event in reader_player_input_event.read() {
        let player_input = player_input_event.0;
        let client_id = player_input_event.1;

        if let Some(player_entity) = lobby.players.get(&client_id) {
            command.entity(*player_entity).insert(player_input);
        }
    }
}

// TODO: Rename possibly on_player_command
pub fn server_receive_player_command_system(
    mut writer_equippable_use: EventWriter<EquippedUse>,
    mut reader_player_command_event: EventReader<ClientSentCommandEvent>,
    lobby: ResMut<ServerLobby>,
) {
    for player_command_event in reader_player_command_event.read() {
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

pub fn handle_movement_input(
    mut writer_play_animation: EventWriter<PlayAnimationEvent>,
    mut query: Query<(&PlayerInput, &mut Velocity, &mut EntityState, Entity), Without<Death>>,
) {
    for (player_input, mut vel, mut state, entity) in &mut query {
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
            writer_play_animation.send(PlayAnimationEvent::new(entity, "Walk"));
        } else {
            *state = crate::enums::EntityState::Idle;
            writer_play_animation.send(PlayAnimationEvent::new(entity, "Idle"));
        }

        vel.vector.x = force.x * *vel.current_speed;
        vel.vector.y = force.y * *vel.current_speed;
    }
}

pub fn handle_deck_input(
    mut writer_draw_card_event: EventWriter<DrawCardEvent>,
    query: Query<(&PlayerInput, &Hand, &HandSize, Entity), Without<Death>>,
    is_shuffled: Query<&Library, With<Shuffled>>,
) {
    for (player_input, hand, hand_size, entity) in &mut query.iter() {
        if player_input.draw {
            if let Err(_) = is_shuffled.get(entity) {
                continue;
            }

            if hand.0.len() > hand_size.0 {
                continue;
            }

            let draw_card_event = if hand.0.len() <= 0 {
                DrawCardEvent::new_to_max(entity)
            } else {
                DrawCardEvent::new(entity, 1)
            };

            writer_draw_card_event.send(draw_card_event);
        }
    }
}
