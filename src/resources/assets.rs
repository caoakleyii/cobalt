use std::collections::HashMap;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext},
    prelude::{Asset, Deref, Handle, Resource},
    reflect::{TypePath, TypeUuid},
    sprite::TextureAtlas,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};

use crate::enums::{CollisionGroups, EntityState, Equipment, Sprites};

#[derive(Resource, Default, Deref)]
pub struct AssetLoading(pub u32);

#[derive(Resource, Default)]
pub struct AssetHandler {
    pub textures: HashMap<Sprites, (TextureAtlas, Vec<AnimationConfig>, Option<HitboxConfig>)>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct AssetsConfig {
    pub sprites: SpritesConfig,
    pub stats: StatsConfig,
}

// SPRITE CONFIG
#[derive(Serialize, Deserialize)]
pub struct SpritesConfig {
    pub textures: HashMap<Sprites, SpriteConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteConfig {
    pub name: Sprites,
    pub path: String,
    pub width: f32,
    pub height: f32,
    pub columns: i32,
    pub rows: i32,
    pub animations: Vec<AnimationConfig>,
    pub hitbox: Option<HitboxConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimationConfig {
    pub name: EntityState,
    pub start_index: i32,
    pub end_index: i32,
    pub should_loop: bool,
    pub is_default: bool,
    pub frame_speed: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct HitboxConfig {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}

// TEXT ASSET
#[derive(Resource, Debug)]
pub struct AssetConfigTextHandler {
    pub handle: Handle<TextAsset>,
}

#[derive(Debug, TypeUuid, TypePath, Asset)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
pub struct TextAsset(pub String);

#[derive(Default)]
pub struct TextLoader;

impl AssetLoader for TextLoader {
    type Asset = TextAsset;

    type Settings = ();

    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::<u8>::new();
            reader.read_to_end(&mut bytes).await?;
            let data_str = std::str::from_utf8(&bytes).expect("Could not parse the asset config.");
            let asset = TextAsset(data_str.into());
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json", "toml"]
    }
}

// STATS CONFIG
#[derive(Serialize, Deserialize)]
pub struct StatsConfig {
    pub equipment: HashMap<Equipment, EquipmentStatsConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentStatsConfig {
    pub name: Equipment,
    pub magazine: u32,
    pub max_magazine: u32,
    pub fire_rate: f32,
    pub reload_time: f32,
    pub damage: u32,
    pub spray: f32,
    pub projectile_type: Sprites,
    pub projectile_speed: f32,
    pub projectile_size: f32,
    pub projectile_per_shot: u32,
    pub range: u32,
    pub layers: Vec<CollisionGroups>,
    pub masks: Vec<CollisionGroups>,
}
