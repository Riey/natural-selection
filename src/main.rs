mod dna;

mod component;
mod constants;
mod resource;
mod system;
mod utils;

use crate::system::NaturalSelectionPlugin;

use bevy::prelude::{AddDefaultPlugins, App};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(NaturalSelectionPlugin::new(150, 50, 1.0))
        .run();
}
