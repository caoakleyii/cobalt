use bevy::prelude::{Children, Query};
use bevy_health_bar::ProgressBar;

use crate::stats::components::Health;

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
