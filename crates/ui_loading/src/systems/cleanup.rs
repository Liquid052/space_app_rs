use ui_core::prelude::Ui;
use crate::prelude::*;
use bevy::prelude::*;

pub fn cleanup_resources(world: &mut World) {
    world.remove_resource::<LoadingUiConfig>();
}

pub fn cleanup_ui(
    loading_bar: Query<Entity, With<LoadingBar>>,
    loading_text: Query<Entity, With<LoadingUi>>,
    mut commands: Commands,
) {
    if let Ok(load_bar_entity) = loading_bar.get_single() {
        commands.entity(load_bar_entity).despawn_recursive();
    }
    if let Ok(text_entity) = loading_text.get_single() {
        commands.entity(text_entity).despawn_recursive();
    }
}
pub fn update_cam(
    loading_cam: Query<Entity, With<LoadingCam>>,
    mut commands: Commands
) {
    if let Ok(cam) = loading_cam.get_single() {
        commands.entity(cam)
            .remove::<LoadingCam>()
            .insert(Ui);
    }
}