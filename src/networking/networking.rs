use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::events::{
    CreatePlayerEvent, DamageEntityEvent, RemovePlayerEvent, SpawnProjectileEvent,
};

/**
 * Server Messages
 *
 * Message types the client receives from theserver
 */
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate(CreatePlayerEvent),
    PlayerRemove(RemovePlayerEvent),
    SpawnProjectile(SpawnProjectileEvent),
    DamageEntity(DamageEntityEvent),
    // DespawnProjectile {
    //     entity: Entity,
    // },
}
