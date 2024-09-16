use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

use crate::simulation::AllSpatialBiosphereInformation;
use crate::utility_functions::integer_math::square_root_64;

pub fn split_blob(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    blob_number: usize
) {
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! Add a condition here to rule out blobs that only contain members of a single colony !!!!!!!!!!!!!!!!!!
    // If the blob has more than one member.
    if all_spatial_biosphere_information.blob_vec[blob_number].blob_members.len() > 1 {
        let mut colony_list: Vec<usize> = Vec::new();

        for organism_index in 1..all_spatial_biosphere_information.blob_vec[
            blob_number
        ].blob_members.len() {
            let organism_number =
                all_spatial_biosphere_information.blob_vec[blob_number].blob_members
                    [organism_index];

            // If the organism is part of a colony.
            if
                all_spatial_biosphere_information.organism_information_vec
                    [organism_number].colony_number > 0
            {
                let colony_blob_number =
                    all_spatial_biosphere_information.colony_vec
                        [
                            all_spatial_biosphere_information.organism_information_vec
                                [organism_number].colony_number
                        ][0];

                // If the colony list doesn't already contain the blob
                if !colony_list.contains(&colony_blob_number) {
                    // Add the blob number to the colony list.
                    colony_list.push(colony_blob_number);
                    // Prepare the blob.
                    all_spatial_biosphere_information.blob_vec[colony_blob_number].in_use = true;
                    all_spatial_biosphere_information.blob_vec[colony_blob_number].blob_members =
                        vec![];
                }

                // Add the organism to the colony list.
                all_spatial_biosphere_information.blob_vec[colony_blob_number].blob_members.push(
                    organism_number
                );

                // If the organism is not part of a colony.
            } else {
                // Assign the organism to it's default blob.
                all_spatial_biosphere_information.organism_information_vec[
                    organism_number
                ].blob_number = organism_number;
                // Set the attributes for that blob
                all_spatial_biosphere_information.blob_vec[organism_number].in_use = true;
                all_spatial_biosphere_information.blob_vec[organism_number].blob_members =
                    vec![organism_number];
                all_spatial_biosphere_information.blob_vec[organism_number].angular_velocity = 0;
                all_spatial_biosphere_information.blob_vec[organism_number].blob_mass =
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].mass;
                all_spatial_biosphere_information.blob_vec[organism_number].center_of_mass_x =
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].x_location;
                all_spatial_biosphere_information.blob_vec[organism_number].center_of_mass_y =
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].y_location;

                // If it is oblong, calculate the moment of inertia.
                if
                    all_spatial_biosphere_information.organism_information_vec
                        [organism_number].oblong
                {
                    let mut moment_of_intertia = 0;

                    for other_circles in all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].other_circle_positions.iter() {
                        moment_of_intertia =
                            (other_circles.distance_from_org_center *
                                other_circles.distance_from_org_center *
                                all_spatial_biosphere_information.organism_information_vec
                                    [organism_number].mass) /
                            ((
                                all_spatial_biosphere_information.organism_information_vec[
                                    organism_number
                                ].other_circle_positions.len() as i32
                            ) +
                                1);
                    }
                    all_spatial_biosphere_information.blob_vec[
                        organism_number
                    ].blob_moment_of_inertia = moment_of_intertia;

                    // Otherwise just assign 1 as a placeholder value of 1.
                } else {
                    all_spatial_biosphere_information.blob_vec[
                        organism_number
                    ].blob_moment_of_inertia = 1;
                }

                // Give the new blob the x and y velocities of the old blob.
                all_spatial_biosphere_information.blob_vec[organism_number].blob_x_velocity =
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity;
                all_spatial_biosphere_information.blob_vec[organism_number].blob_y_velocity =
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity;

                // Then add on the angular velocity.

                // Calculate the distance between the old blob center and the new blob center.
                let x_displacement =
                    all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x -
                    all_spatial_biosphere_information.blob_vec[organism_number].center_of_mass_x;
                let y_displacement =
                    all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x -
                    all_spatial_biosphere_information.blob_vec[organism_number].center_of_mass_x;

                let distance_from_blob_center = square_root_64(
                    (x_displacement as i64) * (x_displacement as i64) +
                        (y_displacement as i64) * (y_displacement as i64)
                ) as i32;

                // Calculate the linear velocity from the distance and angular velocity.
                let linear_velocity =
                    (distance_from_blob_center *
                        all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity) /
                    1000;
                let angle_from_blob_center = deterministic_trig.d_trig.arctangent((
                    (y_displacement * 1000) / x_displacement,
                    1000,
                )).0;

                // Figure out the angle of the linear velocity.
                let angle_of_tangent = if linear_velocity > 0 {
                    angle_from_blob_center + 1571
                } else {
                    angle_from_blob_center - 1571
                };

                // Split the linear velocity into x and y components and add it to the blob velocity.
                all_spatial_biosphere_information.blob_vec[organism_number].blob_x_velocity =
                    linear_velocity * deterministic_trig.d_trig.cosine((angle_of_tangent, 1000)).0 / 1000;
                all_spatial_biosphere_information.blob_vec[organism_number].blob_y_velocity =
                    linear_velocity * deterministic_trig.d_trig.sine((angle_of_tangent, 1000)).0 / 1000;
            }

            // Code to clear the original blob and calculate its new attributes here.

            // Code to calculate the attributes of the split off colony here.
        }
    }
}
