use crate::component::{Creature, SimulationUi};
use crate::resource::SimulationState;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub fn ui_update_system(
    diagnostics: Res<Diagnostics>,
    simulation: Res<SimulationState>,
    creature_query: Query<(&Creature,)>,
    mut ui_query: Query<(&mut Text, &SimulationUi)>,
) {
    let mut gen = [0; 10];
    let mut old = [0; 20];

    for (creature,) in creature_query.iter() {
        gen[creature.generation()] += 1;
        old[creature.old()] += 1;
    }

    if let SimulationState::Running { turn_count, .. } = &*simulation {
        for (mut text, _ui) in ui_query.iter_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.sections[0].value = format!(
                        "TURN: {}, FPS: {}\nGEN: {:?}\nOLD: {:?}",
                        turn_count, average, gen, old
                    );
                }
            }
        }
    }
}
