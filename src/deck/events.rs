use bevy::ecs::{entity::Entity, event::Event};
use serde::{Deserialize, Serialize};

use super::card::components::CardEntity;

#[derive(Debug, Clone, Event, Serialize, Deserialize)]
pub struct ShuffleEvent {
    pub player: Entity,
    pub seed: u64,
}

// TODO: Move to a different module keyword/events.rs
#[derive(Debug, Clone, Event)]
pub struct DrawCardEvent {
    pub player: Entity,
    pub to_max: bool,
    pub amount: Option<u32>,
}

impl DrawCardEvent {
    pub fn new(entity: Entity, amount: u32) -> Self {
        Self {
            player: entity,
            to_max: false,
            amount: Some(amount),
        }
    }

    pub fn new_to_max(entity: Entity) -> Self {
        Self {
            player: entity,
            to_max: true,
            amount: None,
        }
    }
}

#[derive(Debug, Clone, Event)]
pub struct CardDrawnEvent {
    pub player: Entity,
    pub card: CardEntity,
}
