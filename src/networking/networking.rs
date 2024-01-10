use crate::deck::keyword::events::{DamageEntityEvent, SpawnProjectileEvent};
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::player::events::{CreatePlayerEvent, RemovePlayerEvent};

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
}
