use bevy::app::{App, Plugin, Update};

use self::systems::tick_cast_timers;

pub mod components;
pub mod systems;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_cast_timers);
    }
}
