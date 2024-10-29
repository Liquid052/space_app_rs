use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::{Animator, EaseFunction, Tween};
use bevy_tweening::lens::{SpriteColorLens, TextColorLens};
use crate::prelude::*;


pub fn reset_ui_tweens(
    config: Res<LoadingUiConfig>,
    mut commands: Commands,
    sprite_animators: Query<Entity,With<Animator<Sprite>>>,
) {
    sprite_animators.iter().for_each(|ent| {
        commands.entity(ent)
            .remove::<Animator<Sprite>>()
            .insert(Animator::new(Tween::new(
                EaseFunction::QuinticIn,
                Duration::from_secs_f32(0.5),
                SpriteColorLens {
                    start: config.fade_start_color,
                    end:   config.fade_end_color,
                }
            )));
    });
}

pub fn set_ui_finished(
    config: Res<LoadingUiConfig>,
    mut commands: Commands,
    sprite_animators: Query<Entity,With<Animator<Sprite>>>,
    loading_text: Query<Entity, With<LoadingText>>,
) {
    sprite_animators.iter().for_each(|ent| {
        commands.entity(ent)
            .remove::<Animator<Sprite>>()
            .insert(Animator::new(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(config.fade_in_duration),
                SpriteColorLens {
                    start: Color::linear_rgb(0.0, 0.0, 0.0),
                    end: config.fade_end_color,
                }
            ).then(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(config.fade_out_duration),
                SpriteColorLens {
                    start: config.fade_end_color,
                    end: Color::linear_rgb(0.0, 0.0, 0.0),
                }
            ))));
    });

    loading_text.iter().for_each(|ent| {
        commands.entity(ent)
            .insert(Animator::new(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(config.fade_in_duration),
                TextColorLens {
                    start: config.text_color,
                    end: config.text_color,
                    section: 0,
                }
            ).then(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(config.fade_out_duration),
                TextColorLens {
                    start: config.text_color,
                    end: Color::BLACK,
                    section: 0,
                }
            ))));
    });
}