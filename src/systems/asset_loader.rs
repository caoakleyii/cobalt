use std::collections::HashMap;

use bevy::{
    prelude::{
        AssetEvent, AssetServer, Assets, Commands, EventReader, Handle, Image, NextState, Res,
        ResMut, Vec2,
    },
    render::texture::ImageSampler,
    sprite::TextureAtlas,
};

use crate::{
    enums::GameState,
    resources::{AssetConfigTextHandler, AssetHandler, AssetsConfig, TextAsset},
};

pub fn asset_config_loader_sytem(asset_server: Res<AssetServer>, mut commands: Commands) {
    // load assets into asset handler
    let asset_config_handle: Handle<TextAsset> = asset_server.load("assets.json");

    // store handlers into resource indvidually
    commands.insert_resource(AssetConfigTextHandler {
        handle: asset_config_handle,
    });
}

pub fn asset_loader_system(
    asset_config: Res<AssetConfigTextHandler>,
    asset_server: Res<AssetServer>,
    text_assets: Res<Assets<TextAsset>>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if let Some(config_str) = text_assets.get(&asset_config.handle) {
        let asset_config: AssetsConfig =
            serde_json::from_str(&config_str.0).expect("Could not parse the asset config.");

        let mut character_handles = HashMap::new();

        asset_config
            .sprites
            .textures
            .iter()
            .for_each(|(key, sprite_config)| {
                let image_handle: Handle<Image> = asset_server.load(sprite_config.path.clone());

                let texture_atlas = TextureAtlas::from_grid(
                    image_handle,
                    Vec2::new(sprite_config.width, sprite_config.height),
                    sprite_config.columns as usize,
                    sprite_config.rows as usize,
                    None,
                    None,
                );
                character_handles.insert(
                    key.clone(),
                    (
                        texture_atlas,
                        sprite_config.animations.clone(),
                        sprite_config.hitbox,
                    ),
                );
            });

        let asset_handler = AssetHandler {
            textures: character_handles,
        };

        commands.insert_resource(asset_handler);
        commands.insert_resource(asset_config);
        state.set(GameState::Gameloop);
    }
}

pub fn asset_loader_state_system(
    mut image_events: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for event in image_events.read() {
        match event {
            AssetEvent::Added { id } => {
                let image = assets
                    .get_mut(*id)
                    .expect("Failed to retrieve added image.");
                image.sampler = ImageSampler::nearest();
            }
            _ => {}
        }
    }
}
