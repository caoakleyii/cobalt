use bevy::prelude::{Component, Event, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub enum PlayerCommand {
    UseEquipment { cast_at: Vec2 },
    // ChangeEquipment { equipment: Entity },
}
