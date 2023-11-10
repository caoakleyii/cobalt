use bevy::prelude::{Component, Event, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub enum PlayerCommand {
    BasicAttack { cast_at: Vec3 },
}
