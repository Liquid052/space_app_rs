use std::time::Duration;
use bevy::prelude::*;
use decay_engine::prelude::EngineBuildFinished;
use crate::prelude::*;
use crate::systems::{reset_ui_tweens, set_ui_finished};

pub fn update_ui_building(
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    mut commands: Commands,
    config: Res<LoadingUiConfig>
) {
    let Ok(mut texture_atlas) = loading_bar.get_single_mut() else {
        return;
    };
    let Ok(mut loading_text) = loading_text.get_single_mut() else {
        return;
    };

    texture_atlas.index = 3;
    loading_text.sections[0] = "BUILDING".into();
    loading_text.sections[0].style.color = config.text_color;

    let system = commands.register_one_shot_system(reset_ui_tweens);
    commands.run_system(system);
}


pub fn stall_finished_build_ui(
    mut duration: Local<Duration>,
    mut engine_building_finished: ResMut<EngineBuildFinished>,
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    mut commands: Commands,
    time: Res<Time>,
    config: Res<LoadingUiConfig>,
) {
    if duration.is_zero() && **engine_building_finished {
        *duration = Duration::from_secs_f32(config.fade_sum());
        **engine_building_finished = false;

        loading_bar.single_mut().index = 4;
        let section = &mut loading_text.single_mut().sections[0];
        *section = "FINISHED".into();
        section.style.color = config.text_color;

        let system = commands.register_one_shot_system(set_ui_finished);
        commands.run_system(system);
    }

    match duration.checked_sub(time.delta()) {
        Some(new_dur) => {
            *duration = new_dur;
            **engine_building_finished = false;
        }
        _ => {}
    }
}
