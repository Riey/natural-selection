use crate::resource::SimulationState;

use bevy::prelude::*;

pub fn prepare_simulation_system(mut simulation: ResMut<SimulationState>) {
    if let SimulationState::Prepare {
        daily_creature_count,
        daily_food_count,
    } = &mut *simulation
    {
        *simulation = SimulationState::running(*daily_creature_count, *daily_food_count);
    }
}
