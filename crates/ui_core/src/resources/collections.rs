use bevy_asset_loader::prelude::*;
use bevy::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct LoadingBarCollection {
    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 16, columns = 5, rows = 5))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "core/non_commercial/progress_bars.png")]
    #[asset(image(sampler = nearest))]
    pub sprite: Handle<Image>,
}

#[derive(AssetCollection, Resource, Deref, DerefMut)]
pub struct JbFonts {
    #[asset(paths(
    "core/fonts/jbmono_light.ttf",
    "core/fonts/jbmono_medium.ttf",
    "core/fonts/jbmono_bold.ttf",
    "core/fonts/jbmono_extra_bold.ttf"
    ), collection(typed))]
    pub fonts: Vec<Handle<Font>>,
}