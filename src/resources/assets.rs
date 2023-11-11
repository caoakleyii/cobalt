use std::collections::HashMap;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{Handle, Resource},
    reflect::{TypePath, TypeUuid},
    sprite::TextureAtlas,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};

use crate::enums::{Character, EntityState, Equipment};

#[derive(Resource, Default)]
pub struct AssetHandler {
    pub character_textures: HashMap<Character, (TextureAtlas, Vec<AnimationConfig>)>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct AssetsConfig {
    pub sprites: SpritesConfig,
    pub stats: StatsConfig,
}

// SPRITE CONFIG
#[derive(Serialize, Deserialize)]
pub struct SpritesConfig {
    pub characters: HashMap<Character, CharacterConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterConfig {
    pub name: Character,
    pub path: String,
    pub width: f32,
    pub height: f32,
    pub columns: i32,
    pub rows: i32,
    pub animations: Vec<AnimationConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationConfig {
    pub name: EntityState,
    pub start_index: i32,
    pub end_index: i32,
    pub should_loop: bool,
    pub is_default: bool,
    pub frame_speed: f32,
}

// TEXT ASSET
#[derive(Resource, Debug)]
pub struct AssetConfigTextHandler {
    pub handle: Handle<TextAsset>,
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
pub struct TextAsset(pub String);

#[derive(Default)]
pub struct TextLoader;

impl AssetLoader for TextLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            let asset = TextAsset(data_str.into());
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json", "toml"]
    }
}

// STATS CONFIG
#[derive(Serialize, Deserialize)]
pub struct StatsConfig {
    pub equipment: HashMap<Equipment, EquipmentStats>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentStats {
    pub name: Equipment,
    pub magazine: u32,
    pub max_magazine: u32,
    pub fire_rate: f32,
    pub reload_time: f32,
    pub damage: u32,
    pub spray: f32,
    pub projectile_speed: f32,
    pub projectile_size: f32,
    pub projectile_per_shot: u32,
    pub range: u32,
}
