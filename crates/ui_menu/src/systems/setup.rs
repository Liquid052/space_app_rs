use bevy::prelude::*;
use crate::prelude::*;
use std::time::Duration;
use bevy_tweening::*;
use bevy_tweening::lens::SpriteColorLens;
use ui_core::prelude::*;

const DURATION: f32 = 2.0;


pub fn spawn_background(mut commands: Commands, res: Res<UiMenuCollection>) {
    let texture = res.background.clone_weak();

    let sprite_animator = Animator::new(Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs_f32(DURATION),
        SpriteColorLens {
            start: Color::linear_rgb(0.0, 0.0, 0.0),
            end: Color::linear_rgb(1.0, 1.0, 1.0),
        }
    ));

    commands.spawn((SpriteBundle {
        transform: Transform::from_scale(Vec3::splat(3.0)),
        texture,
        ..default()
    }, Ui, UiBackground, StateScoped(InLoadingOrMenu), Name::new("Menu background"), sprite_animator));
}