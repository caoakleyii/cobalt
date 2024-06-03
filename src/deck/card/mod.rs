use bevy::prelude::*;
use bevy_enum_filter::prelude::AddEnumFilter;

use self::{
    enums::{CardTypes, Cards, SubTypes},
    equipment::EquipmentPlugin,
};

pub mod components;
pub mod enums;
pub mod equipment;
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EquipmentPlugin)
            .add_enum_filter::<Cards>()
            .add_enum_filter::<CardTypes>()
            .add_enum_filter::<SubTypes>();
    }
}
