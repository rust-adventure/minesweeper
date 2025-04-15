use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 64,
        tile_size_y = 64,
        columns = 4,
        rows = 4,
        padding_x = 0,
        padding_y = 0
    ))]
    pub tiles_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "tiles.png")]
    pub tiles: Handle<Image>,
}
