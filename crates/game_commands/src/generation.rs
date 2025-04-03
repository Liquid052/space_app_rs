use bevy::ecs::world::Command;
use bevy::prelude::*;

mod scene_tree;

pub struct SpawnSpace;

impl Command for SpawnSpace {
    fn apply(self, world: &mut World) {

        let mut commands = world.commands();


        todo!()
    }
}