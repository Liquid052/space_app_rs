use bevy::ecs::world::Command;
use bevy::prelude::*;
use space_engine::prelude::*;

// pub struct GenerateSceneTree;
//
// impl Command for GenerateSceneTree {
//     fn apply(self, world: &mut World) {
//         let mut commands = world.commands();
//
//         commands.scene_init("run_0");
//
//         let root = commands.scene_root("world")
//             .id();
//
//         let galaxy = commands.scene_node_under(root, PosKey::Pos(IVec2::ONE * 3), "galaxy")
//             .id();
//
//         let star_key = PosKey::Index(0);
//         commands.scene_node_under(galaxy, star_key, "star_system")
//             .enable_scene();
//
//         commands.entity(root);
//     }
// }