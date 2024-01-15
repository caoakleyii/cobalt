use serde::{Deserialize, Serialize};

/**
 *  Sprite Types
 *
 * Types of sprite assets expected to be loaded by asset config loader
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Sprites {
    Skeleton = 0,
    Bullet,
    SmgBullet,
    ShotgunBullet,
    AK47,
}

impl Into<u8> for Sprites {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Sprites {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Skeleton,
            1 => Self::Bullet,
            2 => Self::SmgBullet,
            3 => Self::ShotgunBullet,
            4 => Self::AK47,
            _ => panic!("Invalid sprite type"),
        }
    }
}

/**
 * Equipment Types
 *
 * Types of equipment expected to be loaded by asset config loader
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Equipment {
    AK47,
    Smg,
    Shotgun,
}
