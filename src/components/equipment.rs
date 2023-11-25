use bevy::{
    prelude::{Bundle, Component, Vec2, Vec3},
    time::{Timer, TimerMode},
};

use crate::{enums::Equipment as EquipmentType, resources::EquipmentStats};

use super::{Speed, Velocity};

// BUNDLE NEEDED for equipment and animation etc.
#[derive(Clone, Debug, Bundle)]
pub struct EquipmentBundle {
    pub equipped: Equipped,
}

impl EquipmentBundle {
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
    pub projectile_speed: f32,
    pub projectile_size: f32,
    pub projectile_per_shot: u32,
    pub range: u32,
    pub fire_rate_timer: Timer,
    pub reload_timer: Timer,
}

impl From<&EquipmentStats> for Equipment {
    fn from(value: &EquipmentStats) -> Self {
        Self {
            equipment_type: value.name,
            magazine: value.magazine,
            max_magazine: value.max_magazine,
            fire_rate: value.fire_rate,
            reload_time: value.reload_time,
            damage: value.damage,
            spray: value.spray,
            range: value.range,
            projectile_speed: value.projectile_speed,
            projectile_size: value.projectile_size,
            projectile_per_shot: value.projectile_per_shot,
            fire_rate_timer: Timer::from_seconds(value.fire_rate, TimerMode::Once),
            reload_timer: Timer::from_seconds(value.reload_time, TimerMode::Once),
        }
    }
}

impl Equipment {
    pub fn use_equipment(&mut self, from: &Vec3, at: &Vec2) -> Velocity {
        self.fire_rate_timer.reset();
        self.magazine -= 1;

        let from = Vec2::new(from.x, from.y);
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
