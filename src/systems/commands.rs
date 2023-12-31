use bevy::{
    math::Vec2,
    prelude::{Assets, Children, Commands, EventReader, Quat, Query, Res, ResMut, Transform},
    sprite::TextureAtlas,
    time::Time,
};
use bevy_2d_collisions::{components::CollisionGroup, events::CollisionBegin};
use bevy_health_bar::ProgressBar;
use bevy_renet::renet::RenetServer;

use crate::{
    components::{
        projectile::ProjectileBundle, Animator, Damage, Equipped, Health, ServerProjectileBundle,
        Team, Velocity,
    },
    enums::{CollisionGroups, ServerMessages},
    events::{DamageEntityEvent, EquippedUse, SpawnProjectileEvent},
    networking::ServerChannel,
    resources::{AssetHandler, NetworkEntities},
};

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

pub fn spawn_projectile(
    mut reader_spawn_projectile: EventReader<SpawnProjectileEvent>,
    mut command: Commands,
    asset_handler: Res<AssetHandler>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for spawn_projectile in reader_spawn_projectile.read() {
        let (texture, animations, hitbox_config) = asset_handler
            .textures
            .get(&spawn_projectile.projectile_type.into())
            .expect("Could not find projectile texture in asset handler.");
        let velocity: Velocity = spawn_projectile.velocity.into();
        let mut transform = Transform::from_translation(spawn_projectile.translation.into());
        transform.rotation = Quat::from_rotation_z(velocity.rotation);

        let hitbox_config = hitbox_config.expect("Could not find hitbox config for bullet.");

        let mut projectile = ProjectileBundle::new(
            Animator::import(animations),
            texture_atlases.add(texture.clone()),
            transform,
            velocity,
            Vec2::new(hitbox_config.width, hitbox_config.height),
            CollisionGroup {
                layer: spawn_projectile.layer,
                mask: spawn_projectile.mask,
            },
        );

        projectile.damage = Damage(10.0);

        command.spawn(projectile);
    }
}
pub fn tick_equipment_system(dt: Res<Time>, mut query: Query<&mut Equipped>) {
    query.iter_mut().for_each(|mut equipped| {
        equipped.equipment.fire_rate_timer.tick(dt.delta());
        equipped.equipment.reload_timer.tick(dt.delta());
    });
}

pub fn damage_collision(
    mut events: EventReader<CollisionBegin>,
    mut server: ResMut<RenetServer>,
    dmg_query: Query<&Damage>,
    mut p_query: Query<&mut Health>,
) {
    for event in events.read() {
        let dmg = dmg_query.get(event.entity);
        let health = p_query.get_mut(event.detected);

        if dmg.is_err() {
            continue;
        }

        if health.is_err() {
            continue;
        }

        let dmg = dmg.unwrap();
        let mut health = health.unwrap();

        server.broadcast_message(
            ServerChannel::ServerMessages,
            bincode::serialize(&ServerMessages::DamageEntity(DamageEntityEvent {
                entity: event.detected,
                damage: **dmg,
            }))
            .expect("Could not serialize damage entity message."),
        );
        health.current -= **dmg;
    }
}

pub fn on_damage_entity(
    mut events: EventReader<DamageEntityEvent>,
    mut query: Query<&mut Health>,
    network_mapping: Res<NetworkEntities>,
) {
    for event in events.read() {
        if let Some(entity) = network_mapping.0.get(&event.entity) {
            if let Ok(mut health) = query.get_mut(*entity) {
                health.current -= event.damage;
            }
        }
    }
}

pub fn health_bar_update(
    query: Query<(&Health, &Children)>,
    mut bar_query: Query<&mut ProgressBar>,
) {
    for (health, children) in query.iter() {
        for &child in children.iter() {
            if let Ok(mut bar) = bar_query.get_mut(child) {
                bar.value = health.current;
            }
        }
    }
}
