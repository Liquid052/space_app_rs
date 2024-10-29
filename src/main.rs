extern crate bevy;
extern crate decay_engine;
extern crate bevy_inspector_egui;

use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use decay_engine::prelude::*;
use bevy::prelude::*;
use game::prelude::*;


fn main() {
    App::new()
        // plugins
        .add_plugins(EnginePlugin::new("Test"))
        .add_plugins(GamePlugins)
        .add_plugins(WorldInspectorPlugin::new())
        // resources
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        // systems

        .run();
}


//
fn setup(mut commands: Commands) {
    let mut cam_bundle = Camera2dBundle::new_with_far(100.0);
    cam_bundle.tonemapping = Tonemapping::AcesFitted;
    cam_bundle.camera.hdr = true;

    commands.spawn((cam_bundle,
                    BloomSettings {
                        intensity: 0.3,
                        low_frequency_boost: 0.5,
                        low_frequency_boost_curvature: 0.95,
                        high_pass_frequency: 1.0,
                        prefilter_settings: BloomPrefilterSettings {
                            threshold: 0.1,
                            threshold_softness: 0.3,
                        },
                        composite_mode: BloomCompositeMode::Additive,
                    },
                    Name::new("Cam"),
                    LoadingCam
    ));
}