use bevy::ecs::system::SystemId;
use bevy::prelude::{Commands, Local, Query, Res, State, Text, TextureAtlas, With};
use decay_engine::prelude::LoadingStates;
use crate::prelude::*;
use crate::systems::reset_ui_tweens;

pub fn update_loading_ui(
    config: Res<LoadingUiConfig>,
    mut reset_system: Local<Option<SystemId>>,
    loading_state:  Res<State<LoadingStates>>,
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    mut commands: Commands,
) {
    if reset_system.is_none() {
        *reset_system = Some(commands.register_one_shot_system(reset_ui_tweens));
    }

    let Ok(mut texture_atlas) = loading_bar.get_single_mut() else {
        return;
    };
    let Ok(mut loading_text) = loading_text.get_single_mut() else {
        return;
    };

    let  (index, text) = match **loading_state {
        LoadingStates::ModIndex => (0, "MOD INDEX"),
        LoadingStates::ModsMeta => (1, "MODS META"),
        LoadingStates::ContentLoading |
        LoadingStates::ContentProcessing => (2, "CONTENT LOADING"),
        _ => panic!("invalid state"),
    };

    texture_atlas.index = index;
    loading_text.sections[0] = text.into();
    loading_text.sections[0].style.color = config.text_color;

    commands.run_system(reset_system.unwrap());
}