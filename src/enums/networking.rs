use bevy::prelude::{Component, Entity};
use serde::{Deserialize, Serialize};

use crate::resources::ClientId;

/**
 * Server Messages
 *
 * Message types the client receives from theserver
 */
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        entity: Entity,
        id: ClientId,
        translation: [f32; 3],
    },
    PlayerRemove {
        id: ClientId,
    },
    // SpawnProjectile {
    //     entity: Entity,
    //     translation: [f32; 3],
    // },
    // DespawnProjectile {
    //     entity: Entity,
    // },
}
