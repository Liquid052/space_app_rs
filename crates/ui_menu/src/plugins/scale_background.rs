use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResized;
use decay_engine::prelude::AppState;
use ui_core::prelude::UiBackground;
use crate::systems::*;

pub struct ScaleBackgroundPlugin;

impl Plugin for ScaleBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), scale_background.after(spawn_background))
            .add_systems(PostUpdate, scale_background
                .run_if(any_with_component::<UiBackground>
                    .and_then(on_event::<WindowResized>())
                )
            );
    }
}