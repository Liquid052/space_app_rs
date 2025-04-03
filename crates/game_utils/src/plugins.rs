use crate::systems::fullscreen_switch::fullscreen_switch_windows;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub struct GameUtilsPlugin;

impl Plugin for GameUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fullscreen_switch_windows.run_if(
            input_just_pressed(KeyCode::F11)
        ));
    }
}