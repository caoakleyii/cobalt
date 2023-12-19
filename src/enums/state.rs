use bevy::prelude::{Component, States};
use serde::{Deserialize, Serialize};

/**
 * Entity State Types
 *
 * Used to define animations, and sync
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Default, Component)]
pub enum EntityState {
    #[default]
    Idle,
    Walk,
    Run,
    Shoot,
    Reload,
}

/**
 * Game State Types
 *
 * Used to define current state of the game
 */
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Connecting,
    Gameloop,
}

/**
 * Egocentric Types
 *
 * Up, Down, Left, Right
 */
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy, Default, Component)]
pub enum EgocentricDirection {
    Up,
    Down,
    Left,
    #[default]
    Right,
}
