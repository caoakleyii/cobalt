use bevy::ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Commands, Query},
};

use crate::{
    combat::components::Casted,
    deck::{
        components::{Hand, Library},
        events::CardDrawnEvent,
        keyword::components::Draw,
    },
};

pub fn draw_card(
    mut query: Query<(&Draw, &Casted, Entity)>,
    mut q_player: Query<(&mut Hand, &mut Library)>,
    mut writer_card_drawn: EventWriter<CardDrawnEvent>,
    mut commands: Commands,
) {
    for (draw_card, casted, entity) in &mut query {
        let draw_ammount = draw_card.amount as usize;

        if let Ok((mut hand, mut library)) = q_player.get_mut(casted.casted_by) {
            for card in library.0.drain(0..draw_ammount) {
                writer_card_drawn.send(CardDrawnEvent {
                    player: casted.casted_by,
                    card: card.clone(),
                });
                hand.0.push(card);
            }

            if let Some(mut draw_card_cast) = commands.get_entity(entity) {
                draw_card_cast.despawn();
            }
        }
    }
}

// zoom in idea; doesn't work that well though
// pub fn card_draw_fx(
//     q_local_player_casting: Query<&Children, (With<LocalPlayer>, With<Casting>)>,
//     q_cast_time: Query<(&CastTime, &Children)>,
//     q_card_draw: Query<With<Draw>>,
//     mut q_camera: Query<&mut OrthographicProjection, With<PlayerCamera>>,
// ) {
//     for local_player_entity_children in q_local_player_casting.iter() {
//         for child in local_player_entity_children.iter() {
//             if let Ok((cast_time, children)) = q_cast_time.get(*child) {
//                 for cast_child in children {
//                     if let Ok(_) = q_card_draw.get(*cast_child) {
//                         if let Ok(mut ortho_projection) = q_camera.get_single_mut() {
//                             let zoom_speed = 0.02;
//                             let zoom = cast_time.timer.percent() * zoom_speed;

//                             ortho_projection.scale -= zoom;
//                             if ortho_projection.scale <= 0.15 {
//                                 ortho_projection.scale = 0.15
//                             }

//                             if cast_time.timer.percent_left() <= 0.1 {
//                                 ortho_projection.scale = 0.5
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
