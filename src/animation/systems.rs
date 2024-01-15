use bevy::prelude::*;

use crate::animation::components::Animator;
use crate::enums::EntityState;

pub fn animate_sprites(dt: Res<Time>, mut query: Query<(&mut Animator, &mut TextureAtlasSprite)>) {
    for (mut animator, mut sprite) in &mut query {
        animator.current_animation().timer.tick(dt.delta());
        if animator.current_animation().timer.just_finished() {
            sprite.index = if sprite.index == animator.current_animation().last {
                animator.current_animation().first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn sync_animation_state(
    mut query: Query<(&mut Animator, &mut TextureAtlasSprite, &EntityState)>,
) {
    for (mut animator, mut sprite, entity_state) in &mut query {
        if animator.current_animation != *entity_state {
            animator.play(*entity_state);
            sprite.index = animator.current_animation().first;
        }
    }
}
