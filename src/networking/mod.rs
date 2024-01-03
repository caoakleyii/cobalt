use bevy::prelude::*;

pub mod channels;
pub mod components;
pub mod config;
pub mod models;
pub mod networking;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, _app: &mut App) {}
}
