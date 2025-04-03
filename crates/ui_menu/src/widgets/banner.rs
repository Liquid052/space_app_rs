// use bevy::prelude::*;
//
// #[derive(Component)]
// struct BannerWidget;
//
// pub struct BannerWidgetConfig {
//     label: String
// }
//
// pub trait UiBannerWidgetExt<'w> {
//     fn banner_widget<'a: 'w>(&'a mut self, config: BannerWidgetConfig) -> UiBuilder<'w, Entity>;
// }
//
//
// impl BannerWidgetConfig {
//     pub fn from(label: impl Into<String>) -> Self {
//         Self {
//             label: label.into(),
//         }
//     }
// }
//
//
//
// impl<'w> UiBannerWidgetExt<'w> for UiBuilder<'w, UiRoot> {
//     fn banner_widget<'a: 'w>(
//         &'a mut self,
//         config: BannerWidgetConfig,
//     ) -> UiBuilder<'w, Entity> {
//         self.container((ImageBundle::default(), BannerWidget), |banner| {
//             banner
//                 .style()
//                 .position_type(PositionType::Absolute)
//                 // Center the children (the label) horizontally.
//                 .justify_content(JustifyContent::Center)
//                 .scale(4.0)
//                 // Add a nice looking background image to our widget.
//                 .image(ImageSource::Path("core/textures/name_game.png".into()));
//
//             // And we'll want a customizable label on the banner.
//             let mut label = banner.label(LabelConfig::default());
//
//             label
//                 .style()
//                 // Align the label relative to the top of the banner.
//                 .align_self(AlignSelf::Start)
//                 // Move us a few pixels down so we look nice relative to our font.
//                 .top(Val::Px(20.0));
//
//             // We would like to set a default text style without having to pass in the AssetServer.
//             label.entity_commands().set_text(config.label, None);
//         })
//     }
// }