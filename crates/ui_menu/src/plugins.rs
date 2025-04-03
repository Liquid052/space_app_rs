use bevy::prelude::*;
use space_engine::prelude::*;
use ui_core::prelude::InMainMenu;
use crate::plugins::scale_background::ScaleBackgroundPlugin;
use crate::prelude::UiMenuCollection;
use crate::systems::*;

mod scale_background;

pub struct UiMenuPlugin;

impl Plugin for UiMenuPlugin {
    fn build(&self, app: &mut App) {
        // core collections
        app.load_collection::<UiMenuCollection>()
            // plugins
            .add_plugins(ScaleBackgroundPlugin)
            // systems
            .add_systems(OnEnter(AppState::Menu), spawn_background)
            .add_systems(Update, menu.run_if(in_state(InMainMenu)));
    }
}