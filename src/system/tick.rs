use crate::component::Creature;

use bevy::prelude::*;
use bevy::tasks::prelude::*;

pub fn tick_system(
    mut commands: Commands,
    pool: Res<AsyncComputeTaskPool>,
    time: Res<Time>,
    mut creature_query: Query<(Entity, &mut Creature, &Transform)>,
) {
    let delta = time.delta_seconds;
    creature_query.iter().par_iter(32).for_each(
        &pool,
        move |(creature_entity, mut creature, transform)| {
            if let Err(_) = creature.tick(transform.translation().truncate(), delta) {
                // invalid code
                commands.despawn(creature_entity);
            }
        },
    );
}
