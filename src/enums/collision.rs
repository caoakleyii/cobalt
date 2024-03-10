use bevy::ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub enum CollisionGroups {
    Player = 1,
    Enemy = 2,
    Teammate = 4,
    Projectile = 8,
    #[default]
    TeamAlpha = 16,
    TeamBravo = 32,
    AOE = 64,
}

impl Into<u32> for CollisionGroups {
    fn into(self) -> u32 {
        self as u32
    }
}

impl From<u32> for CollisionGroups {
    fn from(value: u32) -> Self {
        match value {
            1 => CollisionGroups::Player,
            2 => CollisionGroups::Enemy,
            4 => CollisionGroups::Teammate,
            8 => CollisionGroups::Projectile,
            16 => CollisionGroups::TeamAlpha,
            32 => CollisionGroups::TeamBravo,
            _ => CollisionGroups::default(),
        }
    }
}

impl CollisionGroups {
    pub fn enemy_teams(&self) -> u32 {
        match self {
            CollisionGroups::TeamAlpha => CollisionGroups::TeamBravo as u32,
            CollisionGroups::TeamBravo => CollisionGroups::TeamAlpha as u32,
            _ => 0,
        }
    }
}
