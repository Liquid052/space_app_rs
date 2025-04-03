use bevy::prelude::*;
use bevy::color::palettes::basic::BLACK;
use bevy::color::palettes::css::DIM_GRAY;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_tweening::*;
use bevy_tweening::lens::{SpriteColorLens, TextColorLens};
use std::time::Duration;
use ui_core::prelude::*;
use crate::prelude::{LoadingBar, LoadingCam, LoadingText, LoadingUi};

pub fn setup(mut commands: Commands, res: Res<LoadingBarCollection>, jb_fonts: Res<JbFonts>) {
    let texture = res.sprite.clone_weak();

    let sprite_tween = Tween::new(
        EaseFunction::QuinticIn,
        Duration::from_secs_f32(0.70),
        SpriteColorLens {
            start: Color::linear_rgb(0.0, 0.0, 0.0),
            end: Color::linear_rgb(1.0, 1.0, 1.0),
        }
    );

    commands.spawn((SpriteBundle {
        transform: Transform::from_scale(Vec3::splat(3.0)),
        texture,
        ..default()
    }, TextureAtlas {
        layout: res.layout.clone_weak(),
        index: 4,
    }, LoadingBar, Name::new("UI loading bar"), Animator::new(sprite_tween))
    );



    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }, LoadingUi, Name::new("UI text node"))
        )
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Loading",
                                    TextStyle {
                                        color: Color::linear_rgb(0.1, 0.1, 0.1),
                                        font: jb_fonts.fonts[1].clone_weak(),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                )
                                    .with_text_justify(JustifyText::Center)
                                    .with_style(Style {
                                        width: Val::Px(400.0),
                                        height: Val::Px(100.0),
                                        top: Val::Px(80.0),

                                        ..default()
                                    }),
                                LoadingText,
                            ));
                        });
                });
        });
}

pub fn setup_copyright(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a root node for the UI
    commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_self: JustifySelf::Center,
            top: Val::Percent(50.0),
            margin: UiRect::bottom(Val::Px(35.0)), // Margin at the bottom
            ..Default::default()
        },
        ..Default::default()
    },StateScoped(InLoadingOrMenu)))
        .with_children(|parent| {
            // Create the text with tween animation for color change
            let tween = Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(1.5),
                TextColorLens {
                    start: BLACK.into(),
                    end: DIM_GRAY.into(),
                    section: 0,
                },
            );

            // Add text to the parent node
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "test build",
                    TextStyle {
                        font: asset_server.load("core/fonts/jbmono_light.ttf"),
                        font_size: 20.0,
                        color: BLACK.into(),
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert(Animator::new(tween)) // Add the Animator component to animate text color
                .insert(Ui);
    });
}