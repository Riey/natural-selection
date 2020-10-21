use crate::component::Creature;

use bevy::prelude::*;

pub fn movement_system(time: Res<Time>, mut creature_query: Query<(&Creature, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (creature, mut transform) in &mut creature_query.iter() {
        transform.translate((creature.velocity() * delta_seconds).extend(0.0));
    }
}
