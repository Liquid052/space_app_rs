use bevy_asset_loader::prelude::*;
use bevy::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiMenuCollection {
    #[asset(path = "core/non_commercial/menu_background.png")]
    #[asset(image(sampler = nearest))]
    pub background: Handle<Image>
}