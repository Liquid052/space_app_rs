use std::sync::{Arc, RwLock, Weak};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

// aliases
type Node = Arc<RwLock<UiNode>>;

#[derive(Resource, Default)]
pub struct UiTree {
    root: Node,

    hash_map: HashMap<String, Node>,
    current_path: String,
}

#[derive(Default)]
pub struct UiNode {
    name: String,
    // parent: Mutex<Weak<RwLock<Node>>>,          // Weak reference to parent (wrapped in a Mutex for safe mutability)
    // children: Mutex<Vec<Arc<Node>>>,
}