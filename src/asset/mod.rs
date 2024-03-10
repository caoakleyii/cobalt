use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetApp,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    render::texture::Image,
};

use crate::{client::sets::ClientConnected, enums::GameState};

use self::{
    resources::{AssetLoading, TextAsset, TextLoader},
    systems::{asset_config_loader_sytem, asset_loader_state_system, asset_loader_system},
};

pub mod enums;
pub mod resources;
mod systems;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "server") {
            app.init_asset::<Image>();
        }

        app.init_asset::<TextAsset>();
        app.init_asset_loader::<TextLoader>();

        app.insert_resource(AssetLoading::default());

        // Clients
        app.add_systems(Startup, asset_config_loader_sytem);
        app.add_systems(
            Update,
            asset_loader_system
                .run_if(in_state(GameState::Loading))
                .in_set(ClientConnected),
        );
        app.add_systems(Update, asset_loader_state_system);
    }
}
