use std::sync::Arc;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use crate::helpers::style;

pub(crate) fn set_egui_style(mut contexts: EguiContexts) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        panic!("Unable to obtain egui context");
    };

    let style = Arc::new(style());
    ctx.set_style(style);
}