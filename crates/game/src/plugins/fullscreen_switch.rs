use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use crate::systems::fullscreen_switch_windows;

pub struct FullscreenSwitchPlugin;

impl Plugin for FullscreenSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fullscreen_switch_windows.run_if(
            input_just_pressed(KeyCode::F11)
        ));
    }
}