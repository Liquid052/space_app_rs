use bevy::app::AppExit;
use bevy::color::{Color, LinearRgba, Srgba};
use bevy::color::palettes::css::{BLUE, YELLOW};
use bevy::color::palettes::tailwind::{CYAN_400, GRAY_300, GRAY_400};
use bevy::ecs::system::RunSystemOnce;
use bevy::ecs::world::Command;
use bevy::prelude::{Camera, Commands, Entity, EventWriter, Gray, NextState, Query, ResMut, With, World};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::egui::Align2;
use space_engine::prelude::{AppState, Body, CameraManagementExt, FocusMode, Keplerian, Moon, Planet, SpaceCommandsExt, SpaceDepth, SpaceLayer, SpaceShip, Star, TwoBodyBuilder};
use ui_core::prelude::{LoadGameState, SettingsState};

pub fn menu(
    mut contexts: EguiContexts,
    mut exit_ev: EventWriter<AppExit>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut next_load_game: ResMut<NextState<LoadGameState>>,
    mut next_settings: ResMut<NextState<SettingsState>>
) {

    let Some(context) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("SPACE EDIT")
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0,0.0))
        .collapsible(false)
        .max_width(80.0)
        .show(context,|ui| {

            ui.button("Play").clicked()
                .then(|| {
                    next_app_state.set(AppState::InGame { paused: false });
                    commands.add(SpawnWorld1);
                    commands.add(AddFocusMode);
                    commands.add(UpdateMass);
                    commands.set_camera_layer(SpaceLayer);
                });
            //ui.button("WORLD 2").clicked()
            //    .then(|| {
            //        next_app_state.set(AppState::InGame { paused: false });
            //        commands.add(SpawnWorld2);
            //        commands.add(AddFocusMode);
            //        commands.add(UpdateMass);
            //        commands.set_camera_layer(SpaceLayer);
            //    });

            ui.separator();

            ui.button("EXIT").clicked()
                .then(|| exit_ev.send(AppExit::Success));
        });

}
pub struct AddFocusMode;

impl Command for AddFocusMode {
    fn apply(self, world: &mut World) {
        world.run_system_once(add_focus_mode);
    }
}

pub fn add_focus_mode(mut commands: Commands, cam: Query<Entity, With<Camera>>) {
    cam.iter()
        .for_each(|ent| {
            commands.entity(ent)
                .insert(FocusMode::Body("Kerbin".into()));
        });
}


pub struct SpawnWorld1;

impl Command for SpawnWorld1 {
    fn apply(self, world: &mut World) {
        world.run_system_once(setup_space1);
    }
}

pub struct UpdateMass;

impl Command for UpdateMass {
    fn apply(self, world: &mut World) {
        world.run_system_once(update_mass);
    }
}
fn update_mass(mut star: Query<&mut Keplerian>) {
    star.iter_mut()
        .for_each(|(mut keplerian)| {
            keplerian.semi_major_axis += 1.0;
        });
}

pub struct SpawnWorld2;

impl Command for SpawnWorld2 {
    fn apply(self, world: &mut World) {
        world.run_system_once(setup_space2);
    }
}

fn setup_space1(mut commands: Commands) {
    commands.create_space("Test space");
    commands.space_cam_follow("Kerbin");

    commands.add(Star::new("Sol")
        .mass(1.7565459e28)
        .radius(261_600_000.0)
        .color(YELLOW.into())
    );

    commands.add(
        Planet::new("Kerbin")
            .mass(5.2915158e22)
            .radius(600_000.0)
            .color(CYAN_400.into())
            .semi_major_axis(23_599_840_256.0)
    );

    commands.add(
        Moon::new("Mun")
            .mass(9.7599050e20)
            .radius(300_000.0)
            .color(GRAY_400.into())
            .semi_major_axis(18_000_000.0)
            .mean_anomaly_at_epoch(-1.0)
            .orbiting("Kerbin"),
    );

    commands.add(
        Moon::new("Asteroid")
            .mass(9.7599068e20)
            .radius(300_000.0)
            .color(GRAY_300.into())
            .eccentricity(0.03565)
            .semi_major_axis(23608596822.4)
            .argument_of_periapsis(1.845)
            .mean_anomaly_at_epoch(-1.773),
    );

    commands.add(
        SpaceShip::new("Vessel")
            .semi_major_axis(2_000_000.0)
            .mean_anomaly_at_epoch(-0.7)
            .color(Color::srgb(1.0,1.0,0.0).into())
            .orbiting("Kerbin")
    );
}

fn setup_space2(mut commands: Commands) {
    commands.create_space("Test space");
    commands.space_cam_follow("Kerbin + Mun");

    commands.add(
        Star::new("Sol")
            .mass(1.7565459e28)
            .radius(261_600_000.0)
            .color(Color::WHITE)
    );

    commands.add(
        TwoBodyBuilder::new(
            Planet::new("Kerbin")
                .mass(5.2915158e22)
                .radius(700_000.0)
                .color(Srgba::gray(0.7).into())
                .semi_major_axis(23_599_840_256.0)
                .belt(1_000_000.0, 1_00_000.0, Color::srgb(0.0, 0.5, 0.0)),
            Moon::new("Mun")
                .mass(4.2915158e22)
                .radius(500_000.0)
                .color(Srgba::gray(0.7).into())
                .semi_major_axis(28_000_000.0),
        )
            .orbiting("Sol")
            .eccentricity_1(0.0)
    );

    commands.add(
        Moon::new("Mun 2")
            .mass(6.2099068e21)
            .radius(450_000.0)
            .color(Color::LinearRgba(LinearRgba::new(1.0,1.0,0.0,1.0)))
            .eccentricity(0.034)
            .semi_major_axis(23608596822.4)
            .argument_of_periapsis(1.845)
            .mean_anomaly_at_epoch(-1.773)
    );

}
