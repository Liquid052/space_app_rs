use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;


pub trait SetZIndexEntExt<'a> {
    fn z_index_global(
        &'a mut self,
        index: i32,
    ) -> &mut EntityCommands<'a>;
}

impl<'a> SetZIndexEntExt<'a> for EntityCommands<'a> {
    fn z_index_global(
        &'a mut self,
        index: i32,
    ) -> &mut EntityCommands<'a> {
        self.insert(ZIndex::Global(index))
    }
}