use bevy::app::{App, Plugin};

pub mod components;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, _app: &mut App) {}
}
