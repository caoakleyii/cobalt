use core::fmt;
use std::collections::HashMap;

use bevy::{
    asset::Handle,
    prelude::{Bundle, Component, Deref, DerefMut, Transform},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{
        Timer,
        TimerMode::{self, Repeating},
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    asset::resources::AnimationConfig,
    enums::state::EgocentricDirection,
};

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
    pub fn from_seconds(seconds: f32, _should_loop: bool) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Repeating))
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
pub struct AnimatedBundle {
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub direction: Direction,

    pub animator: Animator,
}

impl AnimatedBundle {
    pub fn new(animator: Animator, texture_atlas: Handle<TextureAtlas>) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas,
                ..Default::default()
            },
            animator,
            ..Default::default()
        }
    }

    pub fn with_direction(&mut self, direction: EgocentricDirection) -> &Self {
        self.direction.0 = direction;
        self
    }

    pub fn with_transform(&mut self, transform: Transform) -> &Self {
        self.sprite_sheet_bundle.transform = transform;
        self
    }
}

#[derive(Clone, Component, Default)]
pub struct Animator {
    current_animation: AnimationName,
    pub animations: HashMap<AnimationName, Animation>,
    // pub queued_animations: Vec<EntityState>,
}

impl Animator {
    pub fn import(animations: &Vec<AnimationConfig>) -> Self {
        let mut animator = Animator::default();

        animations.iter().for_each(|animation_config| {
            let animation = Animation {
                name: animation_config.name.clone(),
                first: animation_config.start_index as usize,
                last: animation_config.end_index as usize,
                should_loop: animation_config.should_loop,
                timer: AnimationTimer::from_seconds(
                    animation_config.frame_speed,
                    animation_config.should_loop,
                ),
                interruptable_by: animation_config.interruptable_by.clone(),
                finished: false,
            };

            animator
                .animations
                .insert(animation_config.name.clone(), animation);

            if animation_config.is_default {
                animator.current_animation = animation_config.name.clone();
            }
        });

        animator
    }

    pub fn current_animation(&mut self) -> &mut Animation {
        self.animations
            .get_mut(&self.current_animation)
            .expect(&format!(
                "current animation {:?} not found.",
                self.current_animation
            ))
    }

    pub fn play(&mut self, animation: &AnimationName, sprite: &mut TextureAtlasSprite) {
        if !self.can_be_interrupted_by(animation) {
            return;
        }

        if self.current_animation == *animation {
            return;
        }

        if self.animations.get(animation).is_none() {
            return;
        }

        self.current_animation = animation.clone();
        let current_animation = self.current_animation();
        sprite.index = current_animation.first;
        current_animation.timer.reset();
        current_animation.timer.0.unpause();
        current_animation.finished = false;
    }

    pub fn can_be_interrupted_by(&mut self, animation: &AnimationName) -> bool {
        if self.current_animation().finished {
            return true;
        }

        let interruptables = &self.current_animation().interruptable_by;

        if interruptables.contains(&AnimationName("*".to_string())) {
            return true;
        }

        interruptables.contains(&animation)
    }
}

#[derive(
    Clone, Component, Default, Deref, DerefMut, Serialize, Deserialize, Debug, Eq, PartialEq, Hash,
)]
pub struct AnimationName(pub String);

impl fmt::Display for AnimationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Default)]
pub struct Animation {
    pub name: AnimationName,
    pub first: usize,
    pub last: usize,
    pub should_loop: bool,
    pub timer: AnimationTimer,
    pub interruptable_by: Vec<AnimationName>,
    pub finished: bool,
}
