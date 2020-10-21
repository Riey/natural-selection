use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct GameSprites {
    creature: Handle<ColorMaterial>,
    creature_filled: Handle<ColorMaterial>,
    food: Handle<ColorMaterial>,
}

impl GameSprites {
    pub fn new(
        materials: &mut Assets<ColorMaterial>,
        creature_texture: Handle<Texture>,
        creature_filled_texture: Handle<Texture>,
        food_texture: Handle<Texture>,
    ) -> Self {
        Self {
            creature: materials.add(creature_texture.into()),
            creature_filled: materials.add(creature_filled_texture.into()),
            food: materials.add(food_texture.into()),
        }
    }

    pub fn creature(&self) -> Handle<ColorMaterial> {
        self.creature
    }

    pub fn creature_filled(&self) -> Handle<ColorMaterial> {
        self.creature_filled
    }

    pub fn food(&self) -> Handle<ColorMaterial> {
        self.food
    }
}

#[derive(Clone)]
pub enum SimulationState {
    Prepare {
        creature_count: usize,
        food_count: usize,
        daily_food_count: usize,
        turn_interval: f32,
    },
    Running {
        daily_food_count: usize,
        turn_timer: Timer,
        turn_count: usize,
    },
}

impl SimulationState {
    pub fn prepare(
        creature_count: usize,
        food_count: usize,
        daily_food_count: usize,
        turn_interval: f32,
    ) -> Self {
        SimulationState::Prepare {
            creature_count,
            food_count,
            daily_food_count,
            turn_interval,
        }
    }

    pub fn running(daily_food_count: usize, turn_interval: f32) -> Self {
        SimulationState::Running {
            daily_food_count,
            turn_timer: Timer::from_seconds(turn_interval, true),
            turn_count: 0,
        }
    }
}
