use bevy::{
    prelude::{EventReader, Query, Res},
    time::Time,
};

use crate::{components::Equipped, events::EquippedUse};

pub fn equipment_use_system(
    mut reader_equippable_use: EventReader<EquippedUse>,
    mut query: Query<&mut Equipped>,
) {
    reader_equippable_use.iter().for_each(|equippable_use| {
        if let Ok(mut equipped) = query.get_mut(equippable_use.entity) {
            if !equipped.equipment.fire_rate_timer.finished() {
                return;
            }

            if !equipped.equipment.reload_timer.finished() {
                return;
            }

            if equipped.equipment.empty() {
                // TODO: Play empty sound
                // TODO: write to reload event writer
                return;
            }

            println!("Equipment used!");

            equipped.equipment.fire_rate_timer.reset();
        }
    });
}

pub fn tick_equipment_system(dt: Res<Time>, mut query: Query<&mut Equipped>) {
    query.iter_mut().for_each(|mut equipped| {
        equipped.equipment.fire_rate_timer.tick(dt.delta());
        equipped.equipment.reload_timer.tick(dt.delta());
    });
}
