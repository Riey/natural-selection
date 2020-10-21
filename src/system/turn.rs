use crate::component::{Creature, Food};
use crate::resource::{GameSprites, SimulationState};
use crate::utils::{calculate_random_objects, is_out_of_box};

use bevy::prelude::*;

pub fn turn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut simulation: ResMut<SimulationState>,
    sprites: Res<GameSprites>,
    mut creature_query: Query<(Entity, &mut Creature, &Transform)>,
    mut food_query: Query<(&Food, &Transform)>,
) {
    if let SimulationState::Running {
        daily_creature_count,
        daily_food_count,
        turn_count,
        turn_timer,
    } = &mut *simulation
    {
        turn_timer.tick(time.delta_seconds);

        if !turn_timer.finished {
            return;
        }

        *turn_count += 1;

        // Process creature
        for (creature_entity, mut creature, transform) in &mut creature_query.iter() {
            if creature.will_die() || is_out_of_box(transform.translation()) {
                commands.despawn(creature_entity);
            } else {
                if let Some(child) = creature.try_duplicate() {
                    commands
                        .spawn(SpriteComponents {
                            material: sprites.creature(),
                            sprite: Sprite::new(Creature::INIT_SIZE),
                            transform: transform.clone(),
                            ..Default::default()
                        })
                        .with(child);
                }
            }

            creature.time_pass();
        }

        // Spawn creatures
        let mut creature_iter = creature_query.iter();

        for transform in calculate_random_objects(
            Creature::INIT_X,
            Creature::INIT_Y,
            *daily_creature_count,
            creature_iter
                .iter()
                .map(|(_, _, transform)| transform.translation()),
        ) {
            commands
                .spawn(SpriteComponents {
                    material: sprites.creature(),
                    sprite: Sprite::new(Creature::INIT_SIZE),
                    transform,
                    ..Default::default()
                })
                .with(Creature::new());
        }

        // Spawn foods
        let mut food_iter = food_query.iter();

        for transform in calculate_random_objects(
            Food::INIT_X,
            Food::INIT_Y,
            *daily_food_count,
            food_iter
                .iter()
                .take(*daily_food_count * 2)
                .map(|(_food, transform)| transform.translation()),
        ) {
            commands
                .spawn(SpriteComponents {
                    material: sprites.food(),
                    sprite: Sprite::new(Food::INIT_SIZE),
                    transform,
                    ..Default::default()
                })
                .with(Food::new());
        }
    }
}
