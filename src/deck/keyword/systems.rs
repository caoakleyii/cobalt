use bevy::{
    asset::Assets,
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Quat, Vec2},
    sprite::TextureAtlas,
    transform::components::Transform,
};
use bevy_2d_collisions::{components::CollisionGroup, events::CollisionBegin};
use bevy_renet::renet::RenetServer;

use crate::{
    client::resources::NetworkEntities,
    components::{Animator, Health, Velocity},
    networking::{channels::ServerChannel, networking::ServerMessages},
    resources::AssetHandler,
};

use super::{
    components::{Damage, ProjectileBundle},
    events::{DamageEntityEvent, SpawnProjectileEvent},
};

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
