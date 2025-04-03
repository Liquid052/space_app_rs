use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiSet;
use crate::systems::set_egui_style;

#[derive(Default)]
pub struct UiStylePlugin {
    pub default_egui_style: bool
}



impl Plugin for UiStylePlugin {
    fn build(&self, app: &mut App) {
        if !self.default_egui_style {
            app.add_systems(Startup, set_egui_style.after(EguiSet::InitContexts));
        }
    }
}
