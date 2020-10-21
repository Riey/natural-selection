use crate::component::{Creature, Food};
use crate::resource::{GameSprites, SimulationState};
use crate::utils::calculate_random_objects;

use bevy::prelude::*;

pub fn prepare_simulation_system(
    mut commands: Commands,
    mut simulation: ResMut<SimulationState>,
    sprites: Res<GameSprites>,
) {
    if let SimulationState::Prepare {
        creature_count,
        food_count,
        daily_food_count,
        turn_interval,
    } = &mut *simulation
    {
        for transform in calculate_random_objects(
            Creature::INIT_X * 2,
            Creature::INIT_Y * 2,
            *creature_count,
            std::iter::empty(),
        ) {
            commands
                .spawn(SpriteComponents {
                    material: sprites.creature(),
                    transform,
                    sprite: Sprite::new(Creature::INIT_SIZE),
                    ..Default::default()
                })
                .with(Creature::new());
        }

        for transform in calculate_random_objects(
            Food::INIT_X * 2,
            Food::INIT_Y * 2,
            *food_count,
            std::iter::empty(),
        ) {
            commands
                .spawn(SpriteComponents {
                    material: sprites.food(),
                    transform,
                    sprite: Sprite::new(Food::INIT_SIZE),
                    ..Default::default()
                })
                .with(Food::new());
        }

        *simulation = SimulationState::running(*daily_food_count, *turn_interval);
    }
}
