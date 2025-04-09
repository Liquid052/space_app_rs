extern crate bevy;
extern crate space_engine;
extern crate bevy_inspector_egui;

use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::input::mouse::MouseWheel;
use space_engine::prelude::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use app::prelude::*;


fn main() {
    App::new()
        // plugins
        .add_plugins(EnginePlugin::new("Test")
            .enable_space()
            .set(SpacePlugin {
                draw_enabled: true,
                camera_enabled: false,
                cam_background_enabled: true,
                auto_soi_update: true,
                cam_target: None,
                test: false,
            })
        )
        .add_plugins(AppPlugins)
        .register_type::<FocusMode>()
        // resources
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        // systems
        .add_systems(Update, mouse_wheel_zoom)
        .add_systems(Update, test_windows.run_if(in_state(Running)))
        .run();
}




fn test_windows(
    mut egui_contexts: EguiContexts,
    mut time_scale: ResMut<SpaceTimeScale>,
    names: Query<(Entity, &Name), (With<Body>, With<UniquelyNamed>)>,
    mut cam: Query<&mut FocusMode, With<MainCamera>>,
    mut exit_ev: EventWriter<AppExit>,
) {
    let Some(mut ctx) = egui_contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("Config")
        .default_pos([16.0, 16.0])
        .show(ctx, |ui| {
            ui.heading("Time Control");
            // Display current time scale
            ui.label(format!("Current Time Scale: {:.2}x", time_scale.0));

            // Add a slider for fine control
            ui.add(egui::Slider::new(&mut time_scale.0, 0.0..=100000.0)
                .text("Speed")
                .logarithmic(true));

            // Add some preset buttons
            ui.horizontal(|ui| {
                if ui.button("Pause").clicked() {
                    time_scale.0 = 0.0;
                }
                if ui.button("1x").clicked() {
                    time_scale.0 = 1.0;
                }
                if ui.button("10x").clicked() {
                    time_scale.0 = 10.0;
                }
                if ui.button("100x").clicked() {
                    time_scale.0 = 100.0;
                }
            });

            ui.separator();

            // Display list of bodies for focusing
            ui.heading("Focus Target");

            if let Ok(mut focus_mode) = cam.get_single_mut() {
                for (_entity, name) in names.iter() {
                    let name_str = name.to_string();
                    if ui.selectable_label(
                        matches!(*focus_mode, FocusMode::Body(ref n) if n == &name_str),
                        &name_str
                    ).clicked() {
                        *focus_mode = FocusMode::Body(name_str);
                    }
                }
            }

            ui.separator();
            ui.button("Exit").on_hover_text("Exit the game").clicked()
                .then(|| {
                    exit_ev.send(AppExit::Success);
                });
        });
}


fn mouse_wheel_zoom(
    mut evs: EventReader<MouseWheel>,
    mut cam: Query<&mut OrthographicProjection, With<SpaceLayer>>,
) {
    let Ok(mut orto) = cam.get_single_mut() else {
        return;
    };

    evs.read().for_each(|ev| {
        match orto.scale {
            0.1..=1.0 => orto.scale -= ev.y / 10.0,
            1.0..=10. => orto.scale -= ev.y,
            0.5..=200. => orto.scale -= ev.y * 10.0,
            200.0..=1000.0 => orto.scale -= ev.y * 300.0,
            1000.0..=1000000.0 => orto.scale -= ev.y * 1000.0,
            _ => {},
        }

        orto.scale = orto.scale.clamp(0.1, 1000000.0);
    });
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
                    SpacePos::default(),
                    Name::new("Cam"),
                    RenderLayers::default(),
                    LoadingCam,
                    MainCamera
    ));
}