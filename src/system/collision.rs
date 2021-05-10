use crate::component::{Creature, Food};

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn collision_system(
    mut commands: Commands,
    mut creature_query: Query<(&mut Creature, &Transform, &Sprite)>,
    mut food_query: Query<(Entity, &mut Food, &Transform, &Sprite)>,
) {
    for (mut creature, creature_transform, sprite) in creature_query.iter_mut() {
        let creature_size = sprite.size;

        for (food_entity, mut food, food_transform, sprite) in food_query.iter_mut() {
            let collision = collide(
                creature_transform.translation,
                creature_size,
                food_transform.translation,
                sprite.size,
            );

            if let Some(_collision) = collision {
                // eat
                if creature.try_eat_food(&mut food) {
                    commands.entity(food_entity).despawn_recursive();
                }
            }
        }
    }
}
