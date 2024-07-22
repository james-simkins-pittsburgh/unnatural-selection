use bevy::prelude::*;

// This module will hold the code that updates each graphical entity based on its corresponding simulation organism.
pub fn update_graphical_world(
    biosphere: Query<&crate::simulation::AllBiosphereInformation>,
    mut assigned_graphical_entities: Query<
        (
            &mut crate::graphical_world::MainGraphicsOfOrganism,
            &mut Transform,
            &mut TextureAtlas,
            &mut Visibility,
            &mut Handle<Image>,
        ),
        With<crate::graphical_world::Assigned>
    >,
    texture_atlas_handles: Res<crate::graphical_world::texture_loader::TextureAtlasHandles>
) {
    let biosphere = biosphere.single();

    // This updates the position of the sprite.
    for mut graphical_entity in assigned_graphical_entities.iter_mut() {
        // This calculates the z value and index value in a way that maximizes texture loading efficiency.
        let z_and_index_and_texture_number =
            crate::graphical_world::z_and_index_and_texture_number_calculator::calculate_z_and_index_and_texture_number(
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].main_animation_type,
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].species_type,
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].background
            );

        // This updates the index and the texture / texture layout (if needed).
        if
            graphical_entity.0.texture_number == z_and_index_and_texture_number.2 &&
            graphical_entity.0.entity_initiated
        {
            // This updates the index.
            graphical_entity.2.index = z_and_index_and_texture_number.1;
        } else {

            // This updates the texture (image).
            *graphical_entity.4 =
                texture_atlas_handles.texture_atlas_array[
                    z_and_index_and_texture_number.2
                ].0.clone();
            // This updates the texture atlas layout.
            graphical_entity.2.layout =
                texture_atlas_handles.texture_atlas_array[
                    z_and_index_and_texture_number.2
                ].1.layout.clone();
            // This updates the texture atlas index.
            graphical_entity.2.index = z_and_index_and_texture_number.1;
            // This updates the texture number.
            graphical_entity.0.texture_number = z_and_index_and_texture_number.2;
            // This says that the graphical entity has been initiated.
            graphical_entity.0.entity_initiated = true;
        }

        // This updates the transform of the entity.
        graphical_entity.1.translation = Vec3 {
            x: (
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].x_location as f32
            ) / 50.0,
            y: (
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].y_location as f32
            ) / 50.0,
            z: z_and_index_and_texture_number.0,
        };

        *graphical_entity.3 = Visibility::Visible;

        // Code to update the rotation of the sprite should go here.

        /* =================== NEED CODE HERE!!!! ====================== */
    }
}
