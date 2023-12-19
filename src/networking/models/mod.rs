use bevy::prelude::Entity;
use serde::Deserialize;
use serde::Serialize;

use crate::enums::EntityState;

/// Serializable struct
/// sent over the network to update clients of any
/// synced entities
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    pub translations: Vec<[f32; 3]>,
    pub aim_ats: Vec<[f32; 2]>,
    pub states: Vec<EntityState>,
}
