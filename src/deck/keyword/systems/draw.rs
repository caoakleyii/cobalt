use bevy::ecs::{event::EventReader, system::Query};

use crate::deck::{
    components::{Hand, HandSize, Library},
    events::DrawCardEvent,
};

pub fn draw_card_to_hand(
    mut reader_draw_card: EventReader<DrawCardEvent>,
    mut query: Query<(&mut Hand, &mut Library, &HandSize)>,
) {
    for draw_card_event in reader_draw_card.read() {
        if let Ok((mut hand, mut library, hand_size)) = query.get_mut(draw_card_event.entity) {
            let draw_amount = if let Some(draw_amount) = draw_card_event.amount {
                draw_amount as usize
            } else {
                hand_size.0
            };

            for card in library.0.drain(0..draw_amount) {
                hand.0.push(card);
            }
        }
    }
}
