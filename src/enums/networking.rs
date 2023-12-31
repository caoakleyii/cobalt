use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::events::{
    DamageEntityEvent, PlayerCreateEvent, PlayerRemoveEvent, SpawnProjectileEvent,
};

/**
 * Server Messages
 *
 * Message types the client receives from theserver
 */
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate(PlayerCreateEvent),
    PlayerRemove(PlayerRemoveEvent),
    SpawnProjectile(SpawnProjectileEvent),
    DamageEntity(DamageEntityEvent),
    // DespawnProjectile {
    //     entity: Entity,
    // },
}
