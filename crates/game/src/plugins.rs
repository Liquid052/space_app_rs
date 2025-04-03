use bevy::app::*;
use game_utils::prelude::GameUtilsPlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameUtilsPlugin)
    }
}