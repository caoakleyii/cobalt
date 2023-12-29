use bevy::{
    ecs::event::EventWriter,
    math::Vec2,
    prelude::{Assets, Children, Commands, EventReader, Quat, Query, Res, ResMut, Transform},
    sprite::TextureAtlas,
    time::Time,
};
use bevy_2d_collisions::{components::CollisionGroup, events::CollisionBegin};
use bevy_renet::renet::RenetServer;

use crate::{
    components::{
        projectile::ProjectileBundle, Animator, Equipped, ServerProjectileBundle, Team, Velocity,
    },
    enums::{
        CollisionGroups, ServerMessages,
        Sprites::{self},
    },
    events::{EquippedUse, SpawnProjectileEvent},
    networking::ServerChannel,
    resources::AssetHandler,
};

pub fn equipment_use_system(
    mut reader_equippable_use: EventReader<EquippedUse>,
    mut query: Query<&mut Equipped>,
    mut writer_spawn_projectile: EventWriter<SpawnProjectileEvent>,
    mut server: ResMut<RenetServer>,
    transform_query: Query<&Transform>,
    equipped_children_query: Query<(&Children, &Team)>,
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

                        let mask = if equipped.equipment.mask == CollisionGroups::Enemy as u32 {
                            (*team).enemy_teams()
                        } else if equipped.equipment.mask == CollisionGroups::Teammate as u32 {
                            (**team) as u32
                        } else {
                            equipped.equipment.mask
                        };

                        let event = SpawnProjectileEvent {
                            translation: spawn_point.translation.into(),
                            velocity: velocity.vector.into(),
                            projectile_type: equipped.equipment.projectile_type.into(),
                            layer: equipped.equipment.layer,
                            mask,
                        };
                        let message: Vec<u8> =
                            bincode::serialize(&ServerMessages::SpawnProjectile(event))
                                .expect("Could not serialize spawn projectile message.");

                        writer_spawn_projectile.send(event);
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

        let projectile = ProjectileBundle::new(
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

        command.spawn(projectile);
    }
}

pub fn spawn_projectile_server(
    mut reader_spawn_projectile: EventReader<SpawnProjectileEvent>,
    mut command: Commands,
    asset_handler: Res<AssetHandler>,
) {
    for spawn_projectile in reader_spawn_projectile.read() {
        let (_texture, _animations, hitbox_config) = asset_handler
            .textures
            .get(&spawn_projectile.projectile_type.into())
            .expect("Could not find projectile texture in asset handler.");

        let velocity: Velocity = spawn_projectile.velocity.into();
        let mut transform = Transform::from_translation(spawn_projectile.translation.into());
        transform.rotation = Quat::from_rotation_z(velocity.rotation);

        let hitbox_config = hitbox_config.expect("Could not find hitbox config for bullet.");

        let projectile = ServerProjectileBundle::new(
            transform,
            velocity,
            Vec2::new(hitbox_config.width, hitbox_config.height),
            CollisionGroup {
                layer: spawn_projectile.layer,
                mask: spawn_projectile.mask,
            },
        );

        command.spawn(projectile);
    }
}
pub fn tick_equipment_system(dt: Res<Time>, mut query: Query<&mut Equipped>) {
    query.iter_mut().for_each(|mut equipped| {
        equipped.equipment.fire_rate_timer.tick(dt.delta());
        equipped.equipment.reload_timer.tick(dt.delta());
    });
}

pub fn projectile_collisions(mut events: EventReader<CollisionBegin>, mut command: Commands) {
    for event in events.read() {
        println!("{:?}", event);
    }
}
