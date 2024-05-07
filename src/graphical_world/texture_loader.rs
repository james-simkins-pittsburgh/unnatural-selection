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
        Vec2::new(80.0, 80.0),
        1,
        1,
        Some(Vec2::new(2.0, 2.0)),
        Some(Vec2::new(0.0, 0.0))
    )); 

    texture_atlas.texture_atlas_array = [(image, TextureAtlas {index:0, layout:new_texture_atlas})];
}
