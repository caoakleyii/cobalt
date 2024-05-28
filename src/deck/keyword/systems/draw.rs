// use bevy::ecs::{
//     event::{EventReader, EventWriter},
//     system::{Commands, Query},
// };

// use crate::deck::{
//     components::{Hand, HandSize, Library},
//     events::{CardDrawnEvent, DrawCardEvent},
//     keyword::components::Draw,
// };

// pub fn draw_card_to_hand(
//     mut reader_draw_card: EventReader<DrawCardEvent>,
//     mut writer_card_drawn: EventWriter<CardDrawnEvent>,
//     mut query: Query<(&mut Hand, &mut Library, &HandSize)>,
//     mut commands: Commands,
// ) {
//     for draw_card_event in reader_draw_card.read() {
//         if let Ok((mut hand, mut library, hand_size)) = query.get_mut(draw_card_event.player) {
//             let draw_amount = if let Some(draw_amount) = draw_card_event.amount {
//                 draw_amount as usize
//             } else {
//                 hand_size.0
//             };

//             for card in library.0.drain(0..draw_amount - 1) {
//                 writer_card_drawn.send(CardDrawnEvent {
//                     player: draw_card_event.player,
//                     card: card.clone(),
//                 });
//                 hand.0.push(card);
//             }
//         }
//     }
// }

// pub fn tick_draw
