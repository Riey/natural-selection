use crate::component::Creature;
use bevy::prelude::*;
use rand::thread_rng;
use rand_distr::{Distribution, UnitCircle};

pub fn random_move_system(time: Res<Time>, mut creature_query: Query<(&mut Creature,)>) {
    let mut rng = thread_rng();

    for (mut creature,) in &mut creature_query.iter() {
        creature.move_timer().tick(time.delta_seconds);

        if !creature.move_timer().finished {
            continue;
        }

        let [x, y] = UnitCircle.sample(&mut rng);
        creature.set_velocity(Vec2::new(x, y));
    }
}
