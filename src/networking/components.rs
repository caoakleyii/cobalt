use bevy::prelude::{Bundle, Component};

use crate::enums::EntityState;

/**
 * Synced Entity
 *
 * Component for the network synced entity
*/
#[derive(Component, Default)]
pub struct SyncedEntity;

/**
 * Networked Entity Bundle
 *
 * Dictates that anything using this bundle will be
 * server synced
 */
#[derive(Bundle, Default)]
pub struct NetworkedEntityBundle {
    pub synced_entity: SyncedEntity,

    pub state: EntityState,
}
