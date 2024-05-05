use bevy::prelude::*;

// This module will hold the code that updates each graphical entity based on its corresponding simulation organism.

pub fn update_graphical_world(
    biosphere: Query<&crate::simulation::AllBiosphereInformation>,
    mut assigned_graphical_entities: Query<
        (&mut crate::graphical_world::MainGraphicsOfOrganism, &mut Transform, &mut TextureAtlas),
        With<crate::graphical_world::Assigned>
    >
) {
    let biosphere = biosphere.single();

    // This updates the position of the sprite.
    for mut graphical_entity in assigned_graphical_entities.iter_mut() {
        
        // This calculates the z value and index value in a way that maximizes texture loading efficiency.
        let z_and_index = crate::graphical_world::z_and_index_calculator::calculate_z_and_index(
            biosphere.organism_information_vec
                [graphical_entity.0.corresponsing_organism_number].main_animation_type,
            biosphere.organism_information_vec
                [graphical_entity.0.corresponsing_organism_number].species_type,
            biosphere.organism_information_vec
                [graphical_entity.0.corresponsing_organism_number].background
        );
        
        graphical_entity.1.translation = Vec3 {
            x: (
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].x_location as f32
            ) / 50.0,
            y: (
                biosphere.organism_information_vec
                    [graphical_entity.0.corresponsing_organism_number].y_location as f32
            ) / 50.0,
            z: z_and_index.0,
        };

        // Code to update the rotation of the sprite should go here.

        /* **********************  NEED CODE HERE!!!! ********************** */

        // This updates the index and the texture.

        graphical_entity.2.index = z_and_index.1

        // Need texture update here.
    }
}
