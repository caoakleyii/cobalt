use bevy::prelude::*;

use crate::animation::components::Animator;

use super::events::PlayAnimationEvent;

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
