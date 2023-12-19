use serde::{Deserialize, Serialize};

/**
 *  Sprite Types
 *
 * Types of sprite assets expected to be loaded by asset config loader
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Sprites {
    Skeleton,
    Bullet,
    SmgBullet,
    ShotgunBullet,
    AK47,
}

/**
 * Equipment Types
 *
 * Types of equipment expected to be loaded by asset config loader
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Equipment {
    Rifle,
    Smg,
    Shotgun,
}
