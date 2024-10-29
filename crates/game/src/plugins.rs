use bevy::app::{PluginGroup, PluginGroupBuilder};
use ui::prelude::UiPlugins;
use crate::plugins::fullscreen_switch::FullscreenSwitchPlugin;

mod fullscreen_switch;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(UiPlugins)
            .add(FullscreenSwitchPlugin)
    }
}

