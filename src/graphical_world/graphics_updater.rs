use bevy::prelude::*;

// This module will hold the code that updates each graphical entity based on its corresponding simulation organism.

pub fn update_graphical_world(
    biosphere: Query<&crate::simulation::AllBiosphereInformation>,
    mut assigned_graphical_entities: Query<
        &mut crate::graphical_world::MainGraphicsOfOrganism,
        With<crate::graphical_world::Assigned>
    >
) {
    let biosphere = biosphere.single(); 
    for mut graphical_entity in assigned_graphical_entities.iter_mut() {
        graphical_entity.x_location = biosphere.
        organism_information_vec[
            graphical_entity.corresponsing_organism_number
        ].x_location;
        graphical_entity.y_location = biosphere.
        organism_information_vec[
            graphical_entity.corresponsing_organism_number
        ].y_location;
    }
}
