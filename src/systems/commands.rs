use bevy::{
    prelude::{
        AssetServer, Assets, Children, Commands, EventReader, Handle, Image, Quat, Query, Res,
        ResMut, Transform, Vec2,
    },
    sprite::TextureAtlas,
    time::Time,
};
use bevy_renet::renet::RenetServer;

use crate::{
    components::{
        projectile::ProjectileBundle, Animator, Equipped, ServerProjectileBundle, Velocity,
    },
    enums::{EntityState, ServerMessages},
    events::{EquippedUse, SpawnProjectileEvent},
    networking::ServerChannel,
    resources::AnimationConfig,
};

pub fn equipment_use_system(
    mut reader_equippable_use: EventReader<EquippedUse>,
    mut query: Query<&mut Equipped>,
    mut command: Commands,
    mut server: ResMut<RenetServer>,
    transform_query: Query<&Transform>,
    equipped_children_query: Query<&Children>,
) {
    reader_equippable_use.iter().for_each(|equippable_use| {
        if let Ok(children) = equipped_children_query.get(equippable_use.entity) {
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
                        let message = bincode::serialize(&ServerMessages::SpawnProjectile(
                            SpawnProjectileEvent {
                                translation: spawn_point.translation.into(),
                                velocity: velocity.vector.into(),
                            },
                        ))
                        .expect("Could not serialize spawn projectile message.");

                        command.spawn(ServerProjectileBundle::new(spawn_point, velocity));
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    for spawn_projectile in reader_spawn_projectile.iter() {
        let projectile_handle: Handle<Image> = asset_server.load("projectile.png");
        let texture_atlas =
            TextureAtlas::from_grid(projectile_handle, Vec2::new(32.0, 32.0), 2, 5, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animator = Animator::import(&vec![AnimationConfig {
            name: EntityState::Idle,
            start_index: 2,
            end_index: 2,
            should_loop: true,
            is_default: true,
            frame_speed: 0.1,
        }]);
        let velocity: Velocity = spawn_projectile.velocity.into();
        let mut transform = Transform::from_translation(spawn_projectile.translation.into());
        transform.rotation = Quat::from_rotation_z(velocity.rotation);

        command.spawn(ProjectileBundle::new(
            animator,
            texture_atlas_handle,
            transform,
            velocity,
        ));
    }
}
pub fn tick_equipment_system(dt: Res<Time>, mut query: Query<&mut Equipped>) {
    query.iter_mut().for_each(|mut equipped| {
        equipped.equipment.fire_rate_timer.tick(dt.delta());
        equipped.equipment.reload_timer.tick(dt.delta());
    });
}
