mod collision;
mod life_display;
mod movement;
mod setup;
mod simulation;
mod tick;
mod turn;
mod ui_update;

use crate::constants::BACK_COLOR;
use crate::resource::SimulationState;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::{ClearColor, IntoQuerySystem, Plugin};

use self::{
    collision::collision_system, life_display::life_display_system, movement::movement_system,
    setup::setup, simulation::prepare_simulation_system, tick::tick_system, turn::turn_system,
    ui_update::ui_update_system,
};
use bevy::app::AppBuilder;

pub struct NaturalSelectionPlugin {
    init_simulation_state: SimulationState,
}

impl NaturalSelectionPlugin {
    pub fn new(daily_creature_count: usize, daily_food_count: usize, turn_interval: f32) -> Self {
        Self {
            init_simulation_state: SimulationState::prepare(
                daily_creature_count,
                daily_food_count,
                turn_interval,
            ),
        }
    }
}

impl Plugin for NaturalSelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_resource(ClearColor(BACK_COLOR))
            .add_resource(self.init_simulation_state.clone())
            .add_startup_system(setup.system())
            .add_system(prepare_simulation_system.system())
            .add_system(collision_system.system())
            .add_system(movement_system.system())
            .add_system(tick_system.system())
            .add_system(turn_system.system())
            .add_system(life_display_system.system())
            .add_system(ui_update_system.system());
    }
}
