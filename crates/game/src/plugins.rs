use bevy::app::{App, PluginGroup, PluginGroupBuilder};
use bevy::prelude::{AppExtStates, Plugin};
use ui_loading::plugins::LoadingUiPlugin;
use bevy_tweening::TweeningPlugin;
use decay_engine::prelude::Loading;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LoadingUiPlugin)
            .add(TweeningPlugin)
            .add(StateScoped)
    }
}

struct StateScoped;

impl Plugin for  StateScoped {
    fn build(&self, app: &mut App) {
        app.enable_state_scoped_entities::<Loading>();
    }
}