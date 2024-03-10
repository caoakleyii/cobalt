use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{Children, Query},
    transform::components::Transform,
};
use bevy_health_bar::{ProgressBar, ProgressBarBundle};

use crate::{
    deck::components::{Graveyard, Hand, Library},
    player::events::PlayerSpawnedEvent,
    stats::components::Health,
};

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

pub fn spawn_health_bar(
    mut reader_player_spawned: EventReader<PlayerSpawnedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for player_spawned_event in reader_player_spawned.read() {
        let player_entity = player_spawned_event.entity;
        // Perhaps to move this to a generic system, when ever there's a new object w/ health.
        // but for now we only have players so will worry about it later.
        let transform = Transform::from_xyz(-15.0, 19.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5));
        commands
            .spawn(
                ProgressBarBundle::new(100.0, asset_server.load("ui/health_bar.png"))
                    .with_transform(transform),
            )
            .set_parent(player_entity);
    }
}

pub fn spawn_hud(
    mut reader_player_spawned: EventReader<PlayerSpawnedEvent>,
    mut commands: Commands,
    hud_items: Query<(&Library, &Hand, &Graveyard)>,
    asset_server: Res<AssetServer>,
) {
    for player_spawned_event in reader_player_spawned.read() {
        if !player_spawned_event.local_player {
            continue;
        }

        let player_entity = player_spawned_event.entity;

        let (library, hand, graveyard) = hud_items
            .get(player_entity)
            .expect(format!("Player {:?} missing hud items.", player_entity).as_str());
    }
}
