use crate::component::{Creature, SimulationUi};
use crate::resource::SimulationState;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub fn ui_update_system(
    diagnostics: Res<Diagnostics>,
    simulation: Res<SimulationState>,
    mut creature_query: Query<(&Creature,)>,
    mut ui_query: Query<(&mut Text, &SimulationUi)>,
) {
    let mut gen = [0; 10];
    let mut old = [0; 20];

    for (creature,) in &mut creature_query.iter() {
        gen[creature.generation()] += 1;
        old[creature.old()] += 1;
    }

    if let SimulationState::Running { turn_count, .. } = &*simulation {
        for (mut text, _ui) in &mut ui_query.iter() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    text.value = format!("TURN: {}, FPS: {}\nGEN: {:?}\nOLD: {:?}", turn_count, average, gen, old);
                }
            }
        }
    }
}
