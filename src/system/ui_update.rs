use crate::component::SimulationUi;
use crate::resource::SimulationState;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub fn ui_update_system(
    diagnostics: Res<Diagnostics>,
    simulation: Res<SimulationState>,
    mut ui_query: Query<(&mut Text, &SimulationUi)>,
) {
    if let SimulationState::Running { turn_count, .. } = &*simulation {
        for (mut text, _ui) in &mut ui_query.iter() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.value = format!("TURN: {}, FPS: {}", turn_count, average);
                }
            }
        }
    }
}
