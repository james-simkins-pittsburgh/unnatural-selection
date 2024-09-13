use crate::{
    simulation::AllSpatialBiosphereInformation,
    utility_functions::deterministic_trigonometry::DeterministicTrig,
};

struct MassAndCOM {
    center_of_mass_x: i32,
    center_of_mass_y: i32,
    mass: i32,
}

pub fn apply_collision(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    combination_list: &Vec<usize>,
    mineral_involved: bool,
    deterministic_trig: &DeterministicTrig
) {
    let mut x_momentum = 0;
    let mut y_momentum = 0;
    let mut r_momentum = 0;

    // This sets the new blob number to the first blob in the combination list.
    let new_blob_number = combination_list[0];

    // This calculates the new mass and center of mass.
    let new_mass_and_center_of_mass = calculate_mass_and_center_of_mass(
        &all_spatial_biosphere_information,
        &combination_list
    );

    // If a mineral is not involved, this calculates the new momentum.
    if !mineral_involved {
        calculate_momentum(
            &all_spatial_biosphere_information,
            &combination_list,
            &new_mass_and_center_of_mass,
            &mut x_momentum,
            &mut y_momentum,
            &mut r_momentum,
            &deterministic_trig
        );
    }

    // This is the code to combine all the blobs into the new blob.

    // For every blob being combined
    for blob_index in 1..combination_list.len() {
        // For every organism in that blob
        for organism_index in 0..all_spatial_biosphere_information.blob_vec[
            combination_list[blob_index]
        ].blob_members.len() {
            let organism_number =
                all_spatial_biosphere_information.blob_vec
                    [combination_list[blob_index]].blob_members[organism_index];
            // Change the organism's blob association.
            all_spatial_biosphere_information.organism_information_vec[
                organism_number
            ].blob_number = new_blob_number;
            // Add that organism to the new blob list.
            all_spatial_biosphere_information.blob_vec[new_blob_number].blob_members.push(
                organism_number
            );
        }

        // Clear the blob of members
        all_spatial_biosphere_information.blob_vec[combination_list[blob_index]].blob_members =
            vec![];
        // Mark the blob as not in use.
        all_spatial_biosphere_information.blob_vec[combination_list[blob_index]].in_use = false;
    }
}

fn calculate_mass_and_center_of_mass(
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    combination_list: &Vec<usize>
) -> MassAndCOM {
    let mut sum_of_moments_x = 0;
    let mut sum_of_moments_y = 0;
    let mut sum_of_mass = 0;

    for blob_number in combination_list.iter() {
        sum_of_mass += all_spatial_biosphere_information.blob_vec[*blob_number].blob_mass;
        sum_of_moments_x +=
            all_spatial_biosphere_information.blob_vec[*blob_number].blob_mass *
            all_spatial_biosphere_information.blob_vec[*blob_number].blob_x_velocity;
        sum_of_moments_y +=
            all_spatial_biosphere_information.blob_vec[*blob_number].blob_mass *
            all_spatial_biosphere_information.blob_vec[*blob_number].blob_y_velocity;
    }

    return MassAndCOM {
        center_of_mass_x: sum_of_moments_x / sum_of_mass,
        center_of_mass_y: sum_of_moments_y / sum_of_mass,
        mass: sum_of_mass,
    };
}

fn calculate_momentum(
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    combination_list: &Vec<usize>,
    new_mass_and_center_of_mass: &MassAndCOM,
    mut x_momentum: &mut i32,
    mut y_momentum: &mut i32,
    mut r_momentum: &mut i32,
    deterministic_trig: &DeterministicTrig
) {
    for member_blob_number in combination_list.iter() {
        // all_spatial_biosphere_information.blob_vec[*member_blob_number]

        // This calculates the angle of the line between the two centers of mass compared to the positive x axis.
        let angle_to_center_of_mass = deterministic_trig.d_trig.arctangent((
            (1000 *
                (new_mass_and_center_of_mass.center_of_mass_x -
                    all_spatial_biosphere_information.blob_vec
                        [*member_blob_number].center_of_mass_x)) /
                (new_mass_and_center_of_mass.center_of_mass_y -
                    all_spatial_biosphere_information.blob_vec
                        [*member_blob_number].center_of_mass_y),

            1000,
        ));

        // Uses the rotation matrix. Not sure this is right.
        let translational_component =
            (all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_x_velocity *
                deterministic_trig.d_trig.cosine(angle_to_center_of_mass).0 -
                all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_y_velocity *
                    deterministic_trig.d_trig.sine(angle_to_center_of_mass).0) /
            1000;
        let rotational_component =
            (all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_x_velocity *
                deterministic_trig.d_trig.sine(angle_to_center_of_mass).0 +
                all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_y_velocity *
                    deterministic_trig.d_trig.cosine(angle_to_center_of_mass).0) /
            1000;

        // Break the translational component down into x and y.
        let translational_x_component =
            (translational_component *
                deterministic_trig.d_trig.cosine(angle_to_center_of_mass).0) /
            1000;
        let translational_y_component =
            (translational_component * deterministic_trig.d_trig.sine(angle_to_center_of_mass).0) /
            1000;

        // Add the momentum contribution of the blob to the new combined bob.
        *x_momentum += translational_x_component * all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_mass;
        *y_momentum += translational_y_component * all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_mass;
        *r_momentum += rotational_component * all_spatial_biosphere_information.blob_vec[*member_blob_number].blob_mass;

    }
}
