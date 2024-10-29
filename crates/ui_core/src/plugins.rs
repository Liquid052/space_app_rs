use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;
use decay_engine::prelude::*;
use crate::components::{Ui, UiBackground};
use crate::prelude::{InLoadingOrMenu, InMainMenu, InSettings, JbFonts, LoadingBarCollection};
use crate::states::{InLoadGame, InMenu, LoadGameState, SettingsState};

pub struct UiCorePlugin;

impl Plugin for UiCorePlugin {
    fn build(&self, app: &mut App) {

        app.load_core_collection::<LoadingBarCollection>()
            .load_core_collection::<JbFonts>()
            // states
            .add_sub_state::<LoadGameState>()
            .add_sub_state::<SettingsState>()
            .add_computed_state::<InMenu>()
            .add_computed_state::<InMainMenu>()
            .add_computed_state::<InSettings>()
            .add_computed_state::<InLoadGame>()
            .add_computed_state::<InLoadingOrMenu>()
            .enable_state_scoped_entities::<InLoadingOrMenu>()
            // reflect
            .register_type::<Ui>()
            .register_type::<UiBackground>();
    }
}