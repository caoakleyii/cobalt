use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::Parent,
    time::Time,
};
use bevy_health_bar::ProgressBar;

use super::components::{CastBar, CastTime, Castable, Casted, Casting};

pub fn tick_cast_timers(
    dt: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut CastTime, &mut ProgressBar, &Castable, &Parent, Entity), With<CastBar>>,
) {
    for (mut cast_time, mut progress_bar, castable, caster_entity, entity) in &mut query {
        if cast_time.timer.finished() {
            continue;
        }

        if cast_time.timer.paused() {
            continue;
        }

        cast_time.timer.tick(dt.delta());
        progress_bar.value = cast_time.timer.remaining_secs();

        if cast_time.timer.just_finished() {
            if let Some(mut casted_entity_commands) = commands.get_entity(castable.entity) {
                casted_entity_commands.insert(Casted {
                    casted_by: caster_entity.get(),
                });
            }

            if let Some(mut caster_entity_commands) = commands.get_entity(caster_entity.get()) {
                println!("Remove Casting");
                caster_entity_commands.remove::<Casting>();
            }

            if let Some(mut casting_entity_commands) = commands.get_entity(entity) {
                casting_entity_commands.despawn();
            }
        }
    }
}
