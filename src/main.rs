extern crate bevy;
extern crate decay_engine;
extern crate bevy_asset_loader;
extern crate bevy_inspector_egui;
extern crate bevy_tweening;

use std::ops::Deref;
use std::time::Duration;
use bevy::color::palettes::css::WHITE;
use bevy::ecs::system::SystemId;
use decay_engine::prelude::*;
use bevy::prelude::*;
use bevy::time::common_conditions::{on_timer, repeating_after_delay};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use bevy_tweening::*;
use bevy_tweening::lens::{SpriteColorLens, TextColorLens};

#[derive(Component)]
struct ExampleProgress;

#[derive(Component)]
struct ExampleUpdateSections(pub Timer);

#[derive(AssetCollection, Resource)]
struct LoadingBarCollection {
    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 16, columns = 5, rows = 5))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "core/textures/progress_bars.png")]
    #[asset(image(sampler = nearest))]
    pub sprite: Handle<Image>,


}

#[derive(AssetCollection, Resource, Deref, DerefMut)]
struct JbFonts {
    #[asset(paths(
        "core/fonts/jbmono_light.ttf",
        "core/fonts/jbmono_medium.ttf",
        "core/fonts/jbmono_bold.ttf",
        "core/fonts/jbmono_extra_bold.ttf"
        ), collection(typed))]
    fonts: Vec<Handle<Font>>,
}

#[derive(Component)]
struct LoadingCam;
#[derive(Component)]
struct LoadingBar;
#[derive(Component)]
struct LoadingUi;

#[derive(Component)]
struct LoadingText;

fn engine_building_done(res: Res<EngineBuilding>) -> bool {
    res.0
}
fn block_building_state(mut building: ResMut<EngineBuilding>,) {
    **building = false;
}

fn set_as_finished(
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
) {
    let Ok(mut texture_atlas) = loading_bar.get_single_mut() else {
        return;
    };
    let Ok(mut loading_bar) = loading_text.get_single_mut() else {
        return;
    };

    texture_atlas.index = 4;
    loading_bar.sections[0] = "FINISHED".into();

}

fn main() {
    App::new()
        .add_plugins(EnginePlugin::new("Test"))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(TweeningPlugin)
        .add_plugins(
          StateInspectorPlugin::<LoadingStates>::new()
        )
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .enable_state_scoped_entities::<Loading>()
        .load_core_collection::<LoadingBarCollection>()
        .load_core_collection::<JbFonts>()
        // .add_systems(Update, component_animator_system::<_>.in_set(AnimationSystem::AnimationUpdate))
        // systems
        .add_systems(OnEnter(Loading), setup)
        .add_systems(Update, update_loading_ui
            .run_if(state_changed::<LoadingStates>
                    .and_then(not(in_state(LoadingStates::Finished)))
            )
        )
        .add_systems(Update, block_building
            .run_if(in_state(BuildingStates::Building)
                .and_then(not(
                    repeating_after_delay(Duration::from_secs(3))
                ))
            )
        )
        .add_systems(OnEnter(BuildingStates::Building), ui_building_state)
        .add_systems(PostUpdate, update_building_state_ui
            .run_if(in_state(BuildingStates::Building))
        )
        .add_systems(OnEnter(BuildFinished), cleanup)
        .run();
}

fn block_building(mut engine_building: ResMut<EngineBuilding>) {
    **engine_building = false;
}

fn print_state(res: Res<State<BuildingStates>>) {
    info!("Res: {:?}",*res);
}

fn cleanup(
    loading_cam: Query<Entity, With<LoadingCam>>,
    loading_bar: Query<Entity, With<LoadingBar>>,
    loading_text: Query<Entity, With<LoadingUi>>,
    mut commands: Commands,
) {
    if let Ok(cam) = loading_cam.get_single() {
        commands.entity(cam).despawn_recursive();
    }
    if let Ok(load_bar_entity) = loading_bar.get_single() {
        commands.entity(load_bar_entity).despawn_recursive();
    }
    if let Ok(text_entity) = loading_text.get_single() {
        commands.entity(text_entity).despawn_recursive();
    }

    info!("Cleanup")
}


fn unblock_load(mut building: ResMut<EngineBuilding>) {
    **building = true;
}


fn setup(mut commands: Commands, res: Res<LoadingBarCollection>, jb_fonts: Res<JbFonts>) {
    let texture = res.sprite.clone_weak();


    let tween = Tween::new(
        EaseFunction::QuinticIn,
        Duration::from_secs_f32(40.0),
        TextColorLens {
                start: Color::linear_rgb(0.1, 0.1, 0.1),
                end: Color::linear_rgb(1.0, 1.0, 1.0),
            section: 0,
        }
    );

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
      }, LoadingBar, Animator::new(sprite_tween))
    );

    // cam
    let cam = commands.spawn((
        Camera2dBundle::new_with_far(1.0),
        LoadingCam
        )
    ).id();


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
        }, LoadingUi)
        )
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }))
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



fn update_loading_ui(
    mut reset_system: Local<Option<SystemId>>,
    loading_state:  Res<State<LoadingStates>>,
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    mut commands: Commands,
) {
    if reset_system.is_none() {
        *reset_system = Some(commands.register_one_shot_system(reset_timer));
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
    loading_text.sections[0].style.color = TEXT_GRAY;

    commands.run_system(reset_system.unwrap());
}

const TEXT_GRAY:         Color = Color::linear_rgb(0.2, 0.2, 0.2);
const FADE_IN_DURATION:  f32 = 1.0;
const FADE_OUT_DURATION: f32 = 1.0;
const FADE_SUM:          f32 = FADE_IN_DURATION + FADE_OUT_DURATION;


fn ui_building_state(
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    mut commands: Commands,
) {
    let Ok(mut texture_atlas) = loading_bar.get_single_mut() else {
        return;
    };
    let Ok(mut loading_text) = loading_text.get_single_mut() else {
        return;
    };

    texture_atlas.index = 3;
    loading_text.sections[0] = "BUILDING".into();
    loading_text.sections[0].style.color = TEXT_GRAY;

    let system = commands.register_one_shot_system(reset_timer);
    commands.run_system(system);
}

fn update_building_state_ui(
    mut duration: Local<Duration>,
    mut engine_building_finished: ResMut<EngineBuilding>,
    mut loading_bar: Query<&mut TextureAtlas, With<LoadingBar>>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if duration.is_zero() && **engine_building_finished {
        *duration = Duration::from_secs_f32(FADE_SUM);
        **engine_building_finished = false;

        info!("setting up");

        loading_bar.single_mut().index = 4;
        let section = &mut loading_text.single_mut().sections[0];
        *section = "FINISHED".into();
        section.style.color = TEXT_GRAY;

        let system = commands.register_one_shot_system(set_fade_out);
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


fn reset_timer(
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
                    start: Color::linear_rgb(0.4, 0.4, 0.4),
                    end: Color::linear_rgb(1.0, 1.0, 1.0),
                }
            )));
    });
}

fn set_fade_out(
    mut commands: Commands,
    sprite_animators: Query<Entity,With<Animator<Sprite>>>,
    mut loading_text: Query<Entity, With<LoadingText>>,
) {
    sprite_animators.iter().for_each(|ent| {
        commands.entity(ent)
            .remove::<Animator<Sprite>>()
            .insert(Animator::new(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(FADE_IN_DURATION),
                SpriteColorLens {
                    start: Color::linear_rgb(0.0, 0.0, 0.0),
                    end: Color::linear_rgb(1.0, 1.0, 1.0),
                }
            ).then(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(FADE_OUT_DURATION),
                SpriteColorLens {
                    start: Color::linear_rgb(1.0, 1.0, 1.0),
                    end: Color::linear_rgb(0.0, 0.0, 0.0),
                }
            ))));
    });

    loading_text.iter().for_each(|ent| {
        commands.entity(ent)
            .insert(Animator::new(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(FADE_IN_DURATION),
                TextColorLens {
                    start: TEXT_GRAY,
                    end: TEXT_GRAY,
                    section: 0,
                }
            ).then(Tween::new(
                EaseFunction::QuadraticOut,
                Duration::from_secs_f32(FADE_OUT_DURATION),
                TextColorLens {
                    start: TEXT_GRAY,
                    end: Color::BLACK,
                    section: 0,
                }
            ))));
    });
}