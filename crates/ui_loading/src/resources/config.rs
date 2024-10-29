use bevy::color::Color;
use bevy::prelude::Resource;

const TEXT_LUM:       f32 = 0.2;
const FADE_START_LUM: f32 = 0.2;
const FADE_END_LUM:   f32 = 1.0;


#[derive(Resource, Copy, Clone)]
pub struct LoadingUiConfig {
    pub text_color:         Color,
    // 1.0 = 1 second
    pub fade_restart_duration: f32,
    pub fade_start_color:      Color,
    pub fade_end_color:        Color,
    // when finishing
    pub fade_in_duration:      f32,
    pub fade_out_duration:     f32,
}

impl Default for LoadingUiConfig {
    fn default() -> Self {
        Self {
            text_color: Color::linear_rgb(TEXT_LUM, TEXT_LUM, TEXT_LUM),
            fade_restart_duration: 0.2,
            fade_start_color: Color::linear_rgb(FADE_START_LUM, FADE_START_LUM, FADE_START_LUM),
            fade_end_color: Color::linear_rgb(FADE_END_LUM, FADE_END_LUM, FADE_END_LUM),
            fade_in_duration: 1.0,
            fade_out_duration: 1.0,
        }
    }
}
impl LoadingUiConfig {
    pub fn fade_sum(&self) -> f32 {
        self.fade_in_duration + self.fade_out_duration
    }
}