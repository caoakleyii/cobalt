use bevy::prelude::*;

use self::equipment::EquipmentPlugin;

pub mod equipment;
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EquipmentPlugin);
    }
}
