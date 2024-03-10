use bevy::{
    asset::Assets,
    ecs::{
        event::EventReader,
        system::{Commands, Res, ResMut},
    },
    math::{Quat, Vec2},
    sprite::TextureAtlas,
    transform::components::Transform,
};
use bevy_2d_collisions::components::CollisionGroup;

use crate::{
    animation::components::Animator,
    asset::resources::AssetHandler,
    deck::keyword::{
        components::{Damage, ProjectileBundle},
        events::SpawnProjectileEvent,
    },
    physics::components::Velocity,
};

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

        // Refactor to make Damage it's own independent component
        // inserted in to the command spawn
        // not all projectiles will do damage.
        projectile.damage = Damage { amount: 10.0 };

        command.spawn(projectile);
    }
}
