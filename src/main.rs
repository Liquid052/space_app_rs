extern crate bevy;
extern crate decay_engine;

use bevy::app::App;
use decay_engine::prelude::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("Test"))
        .run();
}