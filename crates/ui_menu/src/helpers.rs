use std::time::Duration;
use bevy::color::palettes::basic::BLACK;
use bevy::color::palettes::css::DIM_GRAY;
use bevy_tweening::lens::TextColorLens;
use bevy::prelude::Text;
use bevy_tweening::*;

pub fn start_tween_text() -> Tween<Text> {
    Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticOut,
        // Animation time (one way only; for ping-pong it takes 2 seconds
        // to come back to start).
        Duration::from_secs_f32(0.5),
        // The lens gives the Animator access to the Transform component,
        // to animate it. It also contains the start and end values associated
        // with the animation ratios 0. and 1.
        TextColorLens {
            start: BLACK.into(),
            end: DIM_GRAY.into(),
            section: 0
        },
    )
}