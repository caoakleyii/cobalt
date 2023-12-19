use std::collections::HashMap;

use bevy::{
    prelude::{Bundle, Component, Deref, DerefMut},
    sprite::SpriteSheetBundle,
    time::{
        Timer,
        TimerMode::{self, Repeating},
    },
};

use crate::{
    enums::{state::EgocentricDirection, EntityState},
    resources::AnimationConfig,
};

use super::Object2D;

/**
* Aniamtion Timer
*
* Seperate timer for each animatable object
*/
#[derive(Clone, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, Repeating))
    }
}

impl AnimationTimer {
    pub fn from_seconds(seconds: f32, should_loop: bool) -> Self {
        let mode = if should_loop {
            TimerMode::Repeating
        } else {
            TimerMode::Once
        };
        Self(Timer::from_seconds(seconds, mode))
    }
}

/**
* Direction
*
* Sets the direction of the animation
* Default is `Right``
*/
#[derive(Clone, Component, Default, Deref, DerefMut)]
pub struct Direction(pub EgocentricDirection);

/**
 * Animated 2D Object
 *
 * Base bundle for any animated objects in the world
 */
#[derive(Clone, Bundle, Default)]
pub struct Animated2DObjectBundle {
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub object_2d: Object2D,

    pub direction: Direction,

    pub animator: Animator,
}

#[derive(Clone, Component, Default)]
pub struct Animator {
    pub current_animation: EntityState,
    pub animations: HashMap<EntityState, Animation>,
}

impl Animator {
    pub fn import(animations: &Vec<AnimationConfig>) -> Self {
        let mut animator = Animator::default();

        animations.iter().for_each(|animation_config| {
            let animation = Animation {
                name: animation_config.name,
                first: animation_config.start_index as usize,
                last: animation_config.end_index as usize,
                should_loop: animation_config.should_loop,
                timer: AnimationTimer::from_seconds(
                    animation_config.frame_speed,
                    animation_config.should_loop,
                ),
            };

            animator.animations.insert(animation_config.name, animation);

            if animation_config.is_default {
                animator.current_animation = animation_config.name;
            }
        });

        animator
    }

    pub fn current_animation(&mut self) -> &mut Animation {
        self.animations
            .get_mut(&self.current_animation)
            .expect("current animation not found.")
    }

    pub fn play(&mut self, animation: EntityState) {
        self.current_animation = animation;
    }
}

#[derive(Clone, Default)]
pub struct Animation {
    pub name: EntityState,
    pub first: usize,
    pub last: usize,
    pub should_loop: bool,
    pub timer: AnimationTimer,
}
