use bevy::prelude::{Entity, Event, Vec2};

#[derive(Debug, Event)]
pub struct EquippedUse {
    pub entity: Entity,
    pub at: Vec2,
}
