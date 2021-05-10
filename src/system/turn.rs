use crate::component::{Creature, Food};
use crate::resource::{GameSprites, SimulationState};
use crate::utils::{calculate_random_objects, is_out_of_box};

use bevy::prelude::*;

pub fn turn_system(
    mut commands: Commands,
    mut simulation: ResMut<SimulationState>,
    sprites: Res<GameSprites>,
    mut creature_query: Query<(Entity, &mut Creature, &Transform)>,
    food_query: Query<(&Food, &Transform)>,
) {
    if let SimulationState::Running {
        daily_creature_count,
        daily_food_count,
        turn_count,
    } = &mut *simulation
    {
        *turn_count += 1;

        // Process creature
        for (creature_entity, mut creature, transform) in creature_query.iter_mut() {
            if creature.will_die() || is_out_of_box(transform.translation) {
                commands.entity(creature_entity).despawn_recursive();
            } else {
                if let Some(child) = creature.try_duplicate() {
                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            material: sprites.creature(),
                            sprite: Sprite::new(Creature::INIT_SIZE),
                            transform: transform.clone(),
                            ..Default::default()
                        })
                        .insert(child);
                }
            }

            creature.time_pass();
        }

        //Spawn new creatures
        for transform in calculate_random_objects(
            Creature::INIT_X,
            Creature::INIT_Y,
            *daily_creature_count,
            creature_query
                .iter_mut()
                .map(|(_, _, transform)| transform.translation),
        ) {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    material: sprites.creature(),
                    sprite: Sprite::new(Creature::INIT_SIZE),
                    transform,
                    ..Default::default()
                })
                .insert(Creature::new());
        }

        // Spawn foods
        for transform in calculate_random_objects(
            Food::INIT_X,
            Food::INIT_Y,
            *daily_food_count,
            food_query
                .iter()
                .take(*daily_food_count * 2)
                .map(|(_food, transform)| transform.translation),
        ) {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    material: sprites.food(),
                    sprite: Sprite::new(Food::INIT_SIZE),
                    transform,
                    ..Default::default()
                })
                .insert(Food::new());
        }
    }
}
