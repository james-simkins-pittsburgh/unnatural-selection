use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TextureAtlasHandles {
    pub texture_atlas_handle_array: [(Handle<Image>, Handle<TextureAtlasLayout>); 1],
}

pub fn texture_loader(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>
) {

    // This loads the image part of the texture atlas.
    texture_atlas_handles.texture_atlas_handle_array[1].0 = asset_server.load("scaffold_texture.png");

    // This creates and saves the layout part of the atlas handle.
    texture_atlas_handles.texture_atlas_handle_array[1].1 = texture_atlases.add(
        TextureAtlasLayout::from_grid(
            Vec2::new(80.0, 80.0),
            1,
            1,
            Some(Vec2::new(2.0, 2.0)),
            Some(Vec2::new(0.0, 0.0)),
        )
    );
}
