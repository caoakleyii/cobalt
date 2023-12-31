use bevy::{
    prelude::{Bundle, Component, Handle, Transform, Vec2, Vec3},
    sprite::{SpriteSheetBundle, TextureAtlas},
    time::{Timer, TimerMode},
};

use crate::{
    enums::{Equipment as EquipmentType, Sprites},
    resources::EquipmentStatsConfig,
};

use super::{Animated2DObjectBundle, Animator, Speed, Velocity};

// BUNDLE NEEDED for equipment and animation etc.
#[derive(Clone, Bundle)]
pub struct EquipmentBundle {
    pub equipped: Equipped,

    pub animated_2d_object: Animated2DObjectBundle,
}

impl EquipmentBundle {
    pub fn new(
        equipment: Equipment,
        animator: Animator,
        texture_atlas: Handle<TextureAtlas>,
        transform: Transform,
    ) -> Self {
        Self {
            equipped: Equipped { equipment },
            animated_2d_object: Animated2DObjectBundle {
                animator,
                sprite_sheet_bundle: SpriteSheetBundle {
                    texture_atlas,
                    transform,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

// BUNDLE NEEDED for equipment and animation etc.
#[derive(Clone, Bundle)]
pub struct ServerEquipmentBundle {
    pub equipped: Equipped,
}

impl ServerEquipmentBundle {
    pub fn new(equipment: Equipment) -> Self {
        Self {
            equipped: Equipped { equipment },
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct Equipped {
    pub equipment: Equipment,
}

#[derive(Clone, Debug, Component)]
pub struct Equipment {
    pub equipment_type: EquipmentType,
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
    pub projectile_layer: u32,
    pub projectile_mask: u32,
    pub range: u32,
    pub fire_rate_timer: Timer,
    pub reload_timer: Timer,
}

impl From<&EquipmentStatsConfig> for Equipment {
    fn from(value: &EquipmentStatsConfig) -> Self {
        Self {
            equipment_type: value.name,
            magazine: value.magazine,
            max_magazine: value.max_magazine,
            fire_rate: value.fire_rate,
            reload_time: value.reload_time,
            damage: value.damage,
            spray: value.spray,
            projectile_type: value.projectile_type,
            range: value.range,
            projectile_speed: value.projectile_speed,
            projectile_size: value.projectile_size,
            projectile_per_shot: value.projectile_per_shot,
            projectile_layer: value.layers.iter().fold(0, |acc, x| acc | *x as u32),
            projectile_mask: value.masks.iter().fold(0, |acc, x| acc | *x as u32),
            fire_rate_timer: Timer::from_seconds(value.fire_rate, TimerMode::Once),
            reload_timer: Timer::from_seconds(value.reload_time, TimerMode::Once),
        }
    }
}

impl Equipment {
    pub fn use_equipment(&mut self, from: &Vec3, at: &Vec2) -> Velocity {
        self.fire_rate_timer.reset();
        self.magazine -= 1;

        let from = vec2_from_vec3(from);
        let angle = angle_between(&from, at);

        Velocity {
            base_speed: Speed(self.projectile_speed),
            current_speed: Speed(self.projectile_speed),
            rotation: angle,
            vector: Vec2::new(
                angle.cos() * self.projectile_speed,
                angle.sin() * self.projectile_speed,
            ),
        }
    }

    pub fn empty(&self) -> bool {
        self.magazine == 0
    }

    pub fn reload(&mut self) {
        self.magazine = self.max_magazine;
    }
}

#[derive(Clone, Debug, Component)]
pub struct Inventory {
    pub items: Vec<Equipped>,
}

pub fn angle_between(from: &Vec2, to: &Vec2) -> f32 {
    let y = to.y - from.y;
    let x = to.x - from.x;
    y.atan2(x)
}

pub fn vec2_from_vec3(from: &Vec3) -> Vec2 {
    Vec2::new(from.x, from.y)
}
