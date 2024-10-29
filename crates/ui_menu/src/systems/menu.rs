use bevy::app::AppExit;
use bevy::prelude::{EventWriter, NextState, ResMut};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::egui::Align2;
use decay_engine::prelude::AppState;
use ui_core::prelude::{LoadGameState, SettingsState};

pub fn menu(
    mut contexts: EguiContexts,
    mut exit_ev: EventWriter<AppExit>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_load_game: ResMut<NextState<LoadGameState>>,
    mut next_settings: ResMut<NextState<SettingsState>>
) {

    let Some(context) = contexts.try_ctx_mut() else {
        return;
    };

    egui::Window::new("SPACE EDIT")
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0,0.0))
        .collapsible(false)
        .max_width(50.0)
        .show(context,|ui| {
            ui.horizontal(|ui| {
                ui.button("PLAY").clicked()
                    .then(|| next_app_state.set(AppState::InGame { paused: false }));
                ui.button("LOAD").clicked()
                    .then(|| next_load_game.set(LoadGameState(true)));

            });
            ui.separator();

            ui.button("SETTINGS").clicked()
                .then(|| next_settings.set(SettingsState(true)));
            ui.button("EXIT").clicked()
                .then(|| exit_ev.send(AppExit::Success));
        });

}