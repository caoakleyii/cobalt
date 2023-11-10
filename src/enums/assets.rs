use serde::{Deserialize, Serialize};

/**
 * Asset Types
 *
 * Type of assets expected to be loaded by asset config loader
 */
pub enum AssetType {
    Sprite,
}

/**
 * Sprite Types
 *
 * Type of sprites expected to be loaded by asset config loader
 */
pub enum SpirteType {
    Characters,
}

/**
 *  Character Types
 *
 * Types of characters expected to be loaded by asset config loader
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Character {
    Skeleton,
}
