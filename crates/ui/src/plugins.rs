use bevy::app::{App, Plugin, PluginGroupBuilder};
use bevy::prelude::{AppExtStates, PluginGroup};
use bevy_tweening::TweeningPlugin;
use decay_engine::prelude::Loading;
use ui_core::prelude::UiCorePlugin;
use ui_loading::plugins::UiLoadingPlugin;
use ui_menu::prelude::UiMenuPlugin;
use ui_style::prelude::UiStylePlugin;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(UiCorePlugin)
            .add(UiStylePlugin::default())
            .add(UiLoadingPlugin::default())
            .add(UiMenuPlugin)
            .add(TweeningPlugin)
            .add(StateScopeUiPlugin)

    }
}

struct StateScopeUiPlugin;

impl Plugin for StateScopeUiPlugin {
    fn build(&self, app: &mut App) {
        app.enable_state_scoped_entities::<Loading>();
    }
}