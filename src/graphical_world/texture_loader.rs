use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TextureAtlasHandles {
    pub texture_atlas_array: [(Handle<Image>, TextureAtlas); 1],
}

pub fn texture_loader(
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<TextureAtlasHandles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // This loads the image part of the texture atlas.
    let image = asset_server.load("scaffold_texture.png");

    // This creates and saves the layout part of the atlas handle.
    let new_texture_atlas = texture_atlas_layouts.add (TextureAtlasLayout::from_grid(
        UVec2::new(80, 80),
        1,
        1,
        Some(UVec2::new(4, 4)),
        Some(UVec2::new(4, 4))
    )); 

    texture_atlas.texture_atlas_array = [(image, TextureAtlas {index:0, layout:new_texture_atlas})];
}
