use bevy::ecs::{entity::Entity, event::Event};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Event, Serialize, Deserialize)]
pub struct ShuffleEvent {
    pub entity: Entity,
    pub seed: u64,
}

#[derive(Debug, Clone, Event)]
pub struct DrawCardEvent {
    pub entity: Entity,
    pub to_max: bool,
    pub amount: Option<u32>,
}

impl DrawCardEvent {
    pub fn new(entity: Entity, amount: u32) -> Self {
        Self {
            entity,
            to_max: false,
            amount: Some(amount),
        }
    }

    pub fn new_to_max(entity: Entity) -> Self {
        Self {
            entity,
            to_max: true,
            amount: None,
        }
    }
}
