use bevy::{
    math::Vec2,
    prelude::{Children, Commands, EventReader, Quat, Query, Res, ResMut, Transform},
    time::Time,
};
use bevy_2d_collisions::components::CollisionGroup;
use bevy_renet::renet::RenetServer;

use crate::{
    asset::resources::AssetHandler,
    deck::{
        card::equipment::components::Equipped,
        keyword::{
            components::{Damage, ServerProjectileBundle},
            events::SpawnProjectileEvent,
        },
    },
    enums::CollisionGroups,
    networking::{channels::ServerChannel, networking::ServerMessages},
    player::components::Team,
};

use super::events::EquippedUse;

pub fn equipment_use_system(
    mut reader_equippable_use: EventReader<EquippedUse>,
    mut query: Query<&mut Equipped>,
    mut server: ResMut<RenetServer>,
    mut command: Commands,
    transform_query: Query<&Transform>,
    equipped_children_query: Query<(&Children, &Team)>,
    asset_handler: Res<AssetHandler>,
) {
    reader_equippable_use.read().for_each(|equippable_use| {
        if let Ok((children, team)) = equipped_children_query.get(equippable_use.entity) {
            for &child in children.iter() {
                if let Ok(mut equipped) = query.get_mut(child) {
                    if !equipped.equipment.fire_rate_timer.finished() {
                        return;
                    }

                    if !equipped.equipment.reload_timer.finished() {
                        return;
                    }

                    if equipped.equipment.empty() {
                        return;
                    }

                    if let Ok(transform) = transform_query.get(equippable_use.entity) {
                        let spawn_point = transform.clone();
                        let velocity = equipped
                            .equipment
                            .use_equipment(&spawn_point.translation, &equippable_use.at);

                        let mask = if equipped.equipment.projectile_mask
                            == CollisionGroups::Enemy as u32
                        {
                            (*team).enemy_teams()
                        } else if equipped.equipment.projectile_mask
                            == CollisionGroups::Teammate as u32
                        {
                            (**team) as u32
                        } else {
                            equipped.equipment.projectile_mask
                        };
                        let layer = equipped.equipment.projectile_layer;

                        let event = SpawnProjectileEvent {
                            translation: spawn_point.translation.into(),
                            velocity: velocity.vector.into(),
                            projectile_type: equipped.equipment.projectile_type.into(),
                            layer,
                            mask,
                        };
                        let message: Vec<u8> =
                            bincode::serialize(&ServerMessages::SpawnProjectile(event))
                                .expect("Could not serialize spawn projectile message.");

                        let (_texture, _animations, hitbox_config) = asset_handler
                            .textures
                            .get(&event.projectile_type.into())
                            .expect("Could not find projectile texture in asset handler.");

                        let mut transform = Transform::from_translation(spawn_point.translation);
                        transform.rotation = Quat::from_rotation_z(velocity.rotation);

                        let hitbox_config =
                            hitbox_config.expect("Could not find hitbox config for bullet.");

                        let mut projectile = ServerProjectileBundle::new(
                            transform,
                            velocity,
                            Vec2::new(hitbox_config.width, hitbox_config.height),
                            CollisionGroup { layer, mask },
                        );

                        projectile.damage = Damage(equipped.equipment.damage as f32);

                        command.spawn(projectile);

                        server.broadcast_message(ServerChannel::ServerMessages, message);
                    }
                }
            }
        }
    });
}

pub fn tick_equipment_system(dt: Res<Time>, mut query: Query<&mut Equipped>) {
    query.iter_mut().for_each(|mut equipped| {
        equipped.equipment.fire_rate_timer.tick(dt.delta());
        equipped.equipment.reload_timer.tick(dt.delta());
    });
}
