mod dna;

mod component;
mod constants;
mod resource;
mod system;
mod utils;

use crate::system::NaturalSelectionPlugin;

use bevy::prelude::{App, DefaultPlugins};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(NaturalSelectionPlugin::new(1000, 150))
        .run();
}
