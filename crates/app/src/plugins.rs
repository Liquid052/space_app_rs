use bevy::app::{PluginGroup, PluginGroupBuilder};
use game::prelude::GamePlugins;
use ui::prelude::UiPlugins;


pub struct AppPlugins;

impl PluginGroup for AppPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(UiPlugins)
            .add_group(GamePlugins)
    }
}

