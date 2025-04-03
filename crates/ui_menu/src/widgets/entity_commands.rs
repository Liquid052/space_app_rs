use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub trait BannerWidgetCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color,
    ) -> &mut EntityCommands<'a>;
}

