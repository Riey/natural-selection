use crate::component::{Creature, Food, Wall};

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn collision_system(
    mut commands: Commands,
    mut creature_query: Query<(&mut Creature, &Transform, &Sprite)>,
    mut food_query: Query<(Entity, &mut Food, &Transform, &Sprite)>,
    mut wall_query: Query<(&Wall, &Transform, &Sprite)>,
) {
    for (mut creature, creature_transform, sprite) in &mut creature_query.iter() {
        let creature_size = sprite.size;

        for (food_entity, mut food, food_transform, sprite) in &mut food_query.iter() {
            let collision = collide(
                creature_transform.translation(),
                creature_size,
                food_transform.translation(),
                sprite.size,
            );

            if let Some(_collision) = collision {
                // eat
                if creature.try_eat_food(&mut food) {
                    commands.despawn_recursive(food_entity);
                }
            }
        }

        for (_wall, wall_transform, sprite) in &mut wall_query.iter() {
            let collision = collide(
                creature_transform.translation(),
                creature_size,
                wall_transform.translation(),
                sprite.size,
            );

            if let Some(_collision) = collision {
                // crash
                creature.crash_with_wall();
            }
        }
    }
}
