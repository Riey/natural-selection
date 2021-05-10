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
use bevy::prelude::{
    ClearColor, IntoSystem, ParallelSystemDescriptorCoercion, Plugin, SystemStage,
};
use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::CoreStage};

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
    pub fn new(daily_creature_count: usize, daily_food_count: usize) -> Self {
        Self {
            init_simulation_state: SimulationState::prepare(daily_creature_count, daily_food_count),
        }
    }
}

impl Plugin for NaturalSelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(ClearColor(BACK_COLOR))
            .insert_resource(self.init_simulation_state.clone())
            .add_startup_system(setup.system())
            .add_stage_after(
                CoreStage::Update,
                "turn",
                SystemStage::single(
                    turn_system
                        .system()
                        .with_run_criteria(FixedTimestep::step(0.5)),
                ),
            )
            .add_stage_after(
                "turn",
                "update",
                SystemStage::parallel()
                    .with_system(prepare_simulation_system.system())
                    .with_system(collision_system.system())
                    .with_system(movement_system.system())
                    .with_system(tick_system.system())
                    .with_system(life_display_system.system()),
            )
            .add_system_to_stage(CoreStage::PostUpdate, ui_update_system.system());
    }
}
