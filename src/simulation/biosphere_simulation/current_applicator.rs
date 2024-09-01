use crate::simulation::AllSpatialBiosphereInformation;
use crate::simulation::AllCurrentInformation;
use crate::utility_functions::integer_math::square_root_64;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

pub fn apply_current(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    all_current_information: &AllCurrentInformation,
    blob_number: usize
) {
    let blob_x = i64::from(all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x);
    let blob_y = i64::from(all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_y);
    let blob_mass = i64::from(all_spatial_biosphere_information.blob_vec[blob_number].blob_mass);

    // For every organism in the blob
    for index in 0..all_spatial_biosphere_information.blob_vec[blob_number].blob_members.len() as usize {
        let organism_num = all_spatial_biosphere_information.blob_vec[blob_number].blob_members[index];
        let organism_x = i64::from(all_spatial_biosphere_information.organism_information_vec
            [organism_num].x_location);
        let organism_y = i64::from(all_spatial_biosphere_information.organism_information_vec
            [organism_num].y_location);

        // For every current
        for current_number in 0..all_current_information.current_information_vec.len() as usize {
            let current_x = i64::from(
                all_current_information.current_information_vec[current_number].center_x
            );
            let current_y = i64::from(
                all_current_information.current_information_vec[current_number].center_y
            );
            let current_r = i64::from(
                all_current_information.current_information_vec[current_number].radius
            );

            // If the organism is within the circle that represents the current
            if
                (current_x - organism_x) * (current_x - organism_x) +
                    (current_y - organism_y) * (current_y - organism_y) <= current_r * current_r
            {
                // Then apply the current to the blob
                let current_a = 
                    all_current_information.current_information_vec
                        [current_number].angle_in_radians_times_1000
                ;
                let current_i = i64::from(
                    all_current_information.current_information_vec[current_number].intensity
                );

                if
                    all_spatial_biosphere_information.organism_information_vec
                        [organism_num].part_of_multi_org_blob
                {
                    // In the case of multi-organism blobs, mass and angle to blob center impact rotational dynamics.
                    let org_mass = i64::from(
                        all_spatial_biosphere_information.organism_information_vec[organism_num].mass
                    );
                    let angle_to_blob_center = 
                        deterministic_trig.d_trig.arctangent((
                            (((blob_y - organism_y) * 1000) / (blob_x - organism_x)) as i32,
                            1000,
                        )).0
                    ;

                    // This changes the ordinary velocity for the blob based on the current.
                    // This is the amount of force directed at the center of mass.
                    let translational_velocity_force =
                        (i64::from(
                            deterministic_trig.d_trig.cosine((
                                (current_a - angle_to_blob_center),
                                1000,
                            )).0
                        ) *
                            current_i *
                            org_mass) /
                        1000;
                    // This is the amount of acceleration that force produces in the x direction
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity +=
                        ((i64::from(deterministic_trig.d_trig.cosine((angle_to_blob_center, 1000)).0) *
                            translational_velocity_force) /
                        blob_mass) as i32;
                    1000;
                    // This is the amount of acceleration that force produces in the y direction
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity +=
                        ((i64::from(deterministic_trig.d_trig.sine((angle_to_blob_center, 1000)).0) *
                            translational_velocity_force) /
                        blob_mass) as i32;
                    1000;

                    // This changes the angular velocity for the blob based on the current.
                    // Change in angular velocity (angular acceleration) equals
                    all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity +=
                        // The force of acceleration on the organism
                        ((org_mass *
                            current_i *
                            // Times the angle the line from the organism to the center of mass times 1000
                            i64::from(deterministic_trig.d_trig.sine((
                                current_a - angle_to_blob_center,
                                1000,
                            )).0) *
                            // Times the distance from the organism to the center of mass
                            square_root_64(
                                (blob_x - organism_x) * (blob_x - organism_x) +
                                    (blob_y - organism_y) * (blob_y - organism_y)
                            )) /
                        // Divided the moment of inertia
                        (i64::from(all_spatial_biosphere_information.blob_vec[blob_number].blob_moment_of_inertia) *
                            // times 1000 to cancel out the d_trig function provided angle times 1000
                            1000)) as i32;
                } else {
                    // The acceleration can be added directly if the blob is a single organism.
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity +=
                        ((i64::from(deterministic_trig.d_trig.cosine((current_a, 1000)).0) * current_i) / 1000) as i32;
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity +=
                        ((i64::from(deterministic_trig.d_trig.sine((current_a, 1000)).0) * current_i) / 1000) as i32;
                }
            }
        }
    }
}
