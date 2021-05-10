use bevy::prelude::*;

#[derive(Clone)]
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
        self.creature.clone_weak()
    }

    pub fn creature_filled(&self) -> Handle<ColorMaterial> {
        self.creature_filled.clone_weak()
    }

    pub fn food(&self) -> Handle<ColorMaterial> {
        self.food.clone_weak()
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct DailyCreatureCount(pub usize);
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct DailyFoodCount(pub usize);
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct TurnCount(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SimulationState {
    Prepare,
    Running,
}

