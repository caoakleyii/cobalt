use std::collections::HashMap;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext},
    prelude::{Asset, Deref, Handle, Resource},
    reflect::{TypePath, TypeUuid},
    sprite::TextureAtlas,
    utils::BoxedFuture,
};
use bevy_2d_collisions::components::CollisionGroup;
use serde::{Deserialize, Serialize};

use crate::{
    animation::components::AnimationName,
    asset::enums::Sprites,
    deck::{
        card::{components::Card, enums::Cards},
        enums::Decks,
    },
    enums::CollisionGroups,
};

#[derive(Resource, Default, Deref)]
pub struct AssetLoading(pub u32);

#[derive(Resource, Default)]
pub struct AssetHandler {
    // TODO: Convert textures' value from a Tuple to Struct
    // Perhaps create animation/component Sprite, and Animation
    // impl Into for SpriteConfig and AnimationConfig
    // <Sprites, Sprite>
    pub textures: HashMap<Sprites, (TextureAtlas, Vec<AnimationConfig>, Option<HitboxConfig>)>,

    pub cards: HashMap<Cards, Card>,

    pub decks: HashMap<Decks, Decklist>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct AssetsConfig {
    pub sprites: SpritesConfig,
    pub cards: CardsConfig,
    pub decks: DecksConfig,
}

// SPRITE CONFIG
#[derive(Serialize, Deserialize)]
pub struct SpritesConfig {
    pub textures: HashMap<Sprites, SpriteConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct CardsConfig {
    pub cards: HashMap<Cards, Card>,
}

#[derive(Serialize, Deserialize)]
pub struct DecksConfig {
    pub decks: HashMap<Decks, Decklist>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Decklist {
    pub cards: Vec<Cards>,
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
    pub name: AnimationName,
    pub start_index: i32,
    pub end_index: i32,
    pub should_loop: bool,
    pub is_default: bool,
    pub frame_speed: f32,
    pub interruptable_by: Vec<AnimationName>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct HitboxConfig {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CollisionGroupConfig {
    pub layers: Vec<CollisionGroups>,
    pub masks: Vec<CollisionGroups>,
}

impl Into<CollisionGroup> for CollisionGroupConfig {
    fn into(self) -> CollisionGroup {
        let mut layer = 0;
        let mut mask = 0;
        for group in self.layers {
            layer |= group as u32;
        }
        for group in self.masks {
            mask |= group as u32;
        }
        CollisionGroup { layer, mask }
    }
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
