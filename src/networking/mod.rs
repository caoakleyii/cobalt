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

pub fn is_server() -> impl Condition<()> {
    IntoSystem::into_system(|mut flag: Local<bool>| {
        *flag = cfg!(feature = "server");
        *flag
    })
}

pub fn is_client() -> impl Condition<()> {
    IntoSystem::into_system(|mut flag: Local<bool>| {
        *flag = cfg!(feature = "client");
        *flag
    })
}
