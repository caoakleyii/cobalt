use bevy::prelude::*;

use crate::{animation::components::Animator, asset::resources::AssetHandler};

use super::{
    components::AnimatedBundle,
    events::{PlayAnimationEvent, SpawnSpriteEvent},
};

pub fn animate_sprites(dt: Res<Time>, mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>) {
    for (mut animator, mut sprite) in &mut query {
        let animation = animator.current_animation();

        animation.timer.tick(dt.delta());
        if animation.timer.just_finished() {
            sprite.index = if sprite.index == animation.last {
                if animation.should_loop {
                    animation.first
                } else {
                    animation.finished = true;
                    sprite.index
                }
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn spawn_animation(
    mut reader_spawn_animation: EventReader<SpawnSpriteEvent>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_handler: Res<AssetHandler>,
) {
    for spawn_sprite_event in reader_spawn_animation.read() {
        let sprite = spawn_sprite_event.sprite;

        let (texture, animation, _hitbox_config) = asset_handler
            .textures
            .get(&sprite)
            .expect("unexpected sprite requested.");

        let mut entity = if let Some(entity) = spawn_sprite_event.entity {
            commands.get_entity(entity).expect("Entity not found.")
        } else {
            commands.spawn_empty()
        };

        let mut animation_bundle = AnimatedBundle::new(
            Animator::import(animation),
            texture_atlases.add(texture.clone()),
        );

        if let Some(translation) = spawn_sprite_event.translation {
            animation_bundle.with_transform(Transform::from_translation(translation));
        }

        entity.insert(animation_bundle);

        if let Some(parent) = spawn_sprite_event.parent {
            entity.set_parent(parent);
        }
    }
}

pub fn play_animation(
    mut reader_play_animation: EventReader<PlayAnimationEvent>,
    mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>,
) {
    for play_animation_event in reader_play_animation.read() {
        let animation_name = &play_animation_event.animation;
        let entity = play_animation_event.entity;
        if let Ok((mut animator, mut sprite)) = query.get_mut(entity) {
            animator.play(animation_name, &mut sprite);
        }
    }
}
