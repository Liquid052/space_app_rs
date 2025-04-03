use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::ecs::system::{EntityCommand, EntityCommands};
use bevy::prelude::{Entity, Text, World};

pub struct SetFont(pub String, pub f32, pub Color);

impl EntityCommand for SetFont {
    fn apply(self, entity: Entity, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load(&self.0);

        if let Some(mut text) = world.entity_mut(entity).get_mut::<Text>() {
            for text_section in &mut text.sections {
                text_section.style.font = font.clone();
                text_section.style.font_size = self.1;
                text_section.style.color = self.2;
            }
        }
    }
}

pub trait SetFotEntExt<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> SetFotEntExt<'a> for EntityCommands<'a> {
    fn font(
        &'a mut self,
        font: impl Into<String>,
        size: f32,
        color: Color,
    ) -> &mut EntityCommands<'a> {
        self.add(SetFont(font.into(), size, color))
    }
}