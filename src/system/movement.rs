use crate::component::Creature;

use bevy::prelude::*;

pub fn movement_system(
    time: Res<Time>,
    mut creature_query: Query<(&mut Creature, &mut Transform)>,
) {
    let delta_seconds = f32::min(0.2, time.delta_seconds);

    for (mut creature, mut transform) in &mut creature_query.iter() {
        let movement = creature.velocity() * delta_seconds;
         transform.translation += movement.extend(0.0);

        let distance = movement.length();
        creature.has_moved(distance);
    }
}
