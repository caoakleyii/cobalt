use bevy::ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res, ResMut},
};
use bevy_2d_collisions::events::CollisionBegin;
use bevy_renet::renet::RenetServer;

use crate::{
    client::resources::NetworkEntities,
    combat::components::Health,
    deck::keyword::{components::Damage, events::DamageEntityEvent},
    enums::EntityState,
    networking::{channels::ServerChannel, networking::ServerMessages},
    player::components::Death,
    server::events::SyncEntityEvent,
};

pub fn damage_collision(
    mut events: EventReader<CollisionBegin>,
    mut server: ResMut<RenetServer>,
    mut writer_sync_entity: EventWriter<SyncEntityEvent>,
    mut p_query: Query<(&mut Health, &mut EntityState)>,
    mut command: Commands,
    dmg_query: Query<&Damage>,
) {
    for event in events.read() {
        let dmg = dmg_query.get(event.entity);
        let damagable_result = p_query.get_mut(event.detected);

        if dmg.is_err() {
            continue;
        }

        if damagable_result.is_err() {
            continue;
        }

        let dmg = dmg.unwrap();
        let (mut health, mut entity_state) = damagable_result.unwrap();
        health.current -= dmg.amount;

        if health.current <= 0.0 {
            health.current = 0.0;
            *entity_state = EntityState::Dead;
            if let Some(mut entity_command) = command.get_entity(event.detected) {
                entity_command.insert(Death::default());
            }
        } else {
            println!("Entity hit!");
            *entity_state = EntityState::Hit;
        }

        server.broadcast_message(
            ServerChannel::ServerMessages,
            bincode::serialize(&ServerMessages::DamageEntity(DamageEntityEvent {
                entity: event.detected,
                damage: dmg.amount,
            }))
            .expect("Could not serialize damage entity message."),
        );
        writer_sync_entity.send(SyncEntityEvent {
            entity: event.detected,
        });
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
