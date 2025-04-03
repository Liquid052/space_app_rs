use bevy::prelude::*;

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct Ui;

#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct UiBackground;
