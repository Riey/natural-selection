use crate::component::Creature;

use bevy::prelude::*;

pub fn tick_system(
    mut commands: Commands,
    time: Res<Time>,
    mut creature_query: Query<(Entity, &mut Creature, &Transform)>,
) {
    let delta = time.delta();
    for (creature_entity, mut creature, transform) in creature_query.iter_mut() {
        if let Err(_) = creature.tick(transform.translation.truncate(), delta) {
            // invalid code
            commands.entity(creature_entity).despawn_recursive();
        }
    }
}
