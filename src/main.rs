mod dna;

mod component;
mod constants;
mod resource;
mod system;
mod utils;

use bevy::prelude::{AddDefaultPlugins, App};

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(crate::system::NaturalSelectionPlugin::new(10, 20, 50, 10.0))
        .run();
}
