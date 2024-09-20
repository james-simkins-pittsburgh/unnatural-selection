use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

use crate::simulation::{ AllSpatialBiosphereInformation, BlobRecord };
use crate::utility_functions::integer_math::square_root_64;

pub fn split_blob(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    blob_number: usize
) {
    // If the blob has more than one member.
    if all_spatial_biosphere_information.blob_vec[blob_number].blob_members.len() > 1 {
        // This code checks to see if the blob is all a single colony.
        let mut not_all_one_colony = true;
        if
            all_spatial_biosphere_information.organism_information_vec
                [
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_members[0]
                ].colony_number > 0
        {
            let first_colony_number = all_spatial_biosphere_information.organism_information_vec
                [
                    all_spatial_biosphere_information.blob_vec[blob_number].blob_members[0]
                ].colony_number;

            not_all_one_colony = false;

            for organism_number in all_spatial_biosphere_information.blob_vec[
                blob_number
            ].blob_members.iter() {
                if
                    all_spatial_biosphere_information.organism_information_vec
                        [*organism_number].colony_number != first_colony_number
                {
                    not_all_one_colony = true;

                    break;
                }
            }
        }

        // If it is not all one colony, then it needs to be split up.
        if not_all_one_colony {
            let mut colony_list: Vec<usize> = Vec::new();

            let original_blob = all_spatial_biosphere_information.blob_vec[blob_number].clone();

            all_spatial_biosphere_information.blob_vec[blob_number].in_use = false;
            all_spatial_biosphere_information.blob_vec[blob_number].blob_members = vec![];

            for organism_index in 0..original_blob.blob_members.len() {
                let organism_number = original_blob.blob_members[organism_index];

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

                    // If the colony list doesn't already contain that colony
                    if !colony_list.contains(&colony_blob_number) {
                        // Add the colony blob number to the colony list.
                        colony_list.push(colony_blob_number);
                        // Prepare colony the blob.
                        all_spatial_biosphere_information.blob_vec[
                            colony_blob_number
                        ].in_use = true;
                        all_spatial_biosphere_information.blob_vec[
                            colony_blob_number
                        ].blob_members = vec![];
                    }

                    // Add the organism to the colony list.
                    all_spatial_biosphere_information.blob_vec[
                        colony_blob_number
                    ].blob_members.push(organism_number);

                    // Associate the organism with the blob.
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].blob_number = colony_blob_number;

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
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].part_of_multi_org_blob = false;
                    all_spatial_biosphere_information.organism_information_vec[
                        organism_number
                    ].angle_to_center_of_mass = 0;
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

                    // Calculate the moment of inertia for oblong.
                    if
                        all_spatial_biosphere_information.organism_information_vec
                            [organism_number].oblong
                    {
                        let mut moment_of_inertia = 0;

                        for other_circles in all_spatial_biosphere_information.organism_information_vec[
                            organism_number
                        ].other_circle_positions.iter() {
                            moment_of_inertia =
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
                        ].blob_moment_of_inertia = moment_of_inertia;

                        // Calculate the moment of inertia for a circle.
                    } else {
                        all_spatial_biosphere_information.blob_vec[
                            organism_number
                        ].blob_moment_of_inertia =
                            (all_spatial_biosphere_information.organism_information_vec
                                [organism_number].mass *
                                all_spatial_biosphere_information.organism_information_vec
                                    [organism_number].radius *
                                all_spatial_biosphere_information.organism_information_vec
                                    [organism_number].radius) /
                            2;
                    }

                    calculate_new_velocity(
                        all_spatial_biosphere_information,
                        &deterministic_trig,
                        organism_number,
                        &original_blob
                    );
                }
            }
            // This code calculates attributes for the colony blobs

            for index in 0..colony_list.len() {
                let colony_blob_number = colony_list[index];

                // This calculates mass and center of mass.

                let mut sum_of_moments_x = 0;
                let mut sum_of_moments_y = 0;
                let mut sum_of_mass = 0;

                for organism_number in all_spatial_biosphere_information.blob_vec[
                    colony_blob_number
                ].blob_members.iter() {
                    sum_of_mass += all_spatial_biosphere_information.organism_information_vec
                        [*organism_number].mass;
                    sum_of_moments_x +=
                        all_spatial_biosphere_information.organism_information_vec
                            [*organism_number].x_location * sum_of_mass;
                    sum_of_moments_y +=
                        all_spatial_biosphere_information.organism_information_vec
                            [*organism_number].y_location * sum_of_mass;
                }

                all_spatial_biosphere_information.blob_vec[colony_blob_number].blob_mass =
                    sum_of_mass;
                all_spatial_biosphere_information.blob_vec[colony_blob_number].center_of_mass_x =
                    sum_of_moments_x / sum_of_mass;
                all_spatial_biosphere_information.blob_vec[colony_blob_number].center_of_mass_y =
                    sum_of_moments_y / sum_of_mass;

                // This sets the angle to blob center and part of a multi org blob.

                for organism_number in all_spatial_biosphere_information.blob_vec[
                    colony_blob_number
                ].blob_members.iter() {
                    all_spatial_biosphere_information.organism_information_vec[
                        *organism_number
                    ].angle_to_center_of_mass = if
                        all_spatial_biosphere_information.blob_vec
                            [colony_blob_number].center_of_mass_x -
                            all_spatial_biosphere_information.organism_information_vec
                                [*organism_number].x_location > 0
                    {
                        deterministic_trig.d_trig.arctangent((
                            (all_spatial_biosphere_information.blob_vec
                                [colony_blob_number].center_of_mass_y -
                                all_spatial_biosphere_information.organism_information_vec
                                    [*organism_number].y_location) *
                                1000,
                            all_spatial_biosphere_information.blob_vec
                                [colony_blob_number].center_of_mass_x -
                                all_spatial_biosphere_information.organism_information_vec
                                    [*organism_number].x_location,
                        )).0
                    } else if
                        all_spatial_biosphere_information.blob_vec
                            [colony_blob_number].center_of_mass_x -
                            all_spatial_biosphere_information.organism_information_vec
                                [*organism_number].x_location < 0
                    {
                        3142 +
                            deterministic_trig.d_trig.arctangent((
                                (all_spatial_biosphere_information.blob_vec
                                    [colony_blob_number].center_of_mass_y -
                                    all_spatial_biosphere_information.organism_information_vec
                                        [*organism_number].y_location) *
                                    1000,
                                all_spatial_biosphere_information.blob_vec
                                    [colony_blob_number].center_of_mass_x -
                                    all_spatial_biosphere_information.organism_information_vec
                                        [*organism_number].x_location,
                            )).0
                    } else {
                        if
                            all_spatial_biosphere_information.blob_vec
                                [colony_blob_number].center_of_mass_y -
                                all_spatial_biosphere_information.organism_information_vec
                                    [*organism_number].y_location > 0
                        {
                            1571
                        } else {
                            -1571
                        }
                    };

                    if
                        all_spatial_biosphere_information.blob_vec[
                            colony_blob_number
                        ].blob_members.len() > 1
                    {
                        all_spatial_biosphere_information.organism_information_vec[
                            *organism_number
                        ].part_of_multi_org_blob = true;
                    } else {
                        all_spatial_biosphere_information.organism_information_vec[
                            *organism_number
                        ].part_of_multi_org_blob = false;
                    }
                }

                // This calculates the moment of inertia.
                // This is for the case of a multi-blob colony.
                if colony_list.len() > 1 {
                    let mut moment_of_inertia = 0;

                    for organism_number in all_spatial_biosphere_information.blob_vec[
                        colony_blob_number
                    ].blob_members.iter() {
                        moment_of_inertia +=
                            ((all_spatial_biosphere_information.organism_information_vec
                                [*organism_number].x_location -
                                all_spatial_biosphere_information.blob_vec
                                    [colony_blob_number].center_of_mass_x) *
                                (all_spatial_biosphere_information.organism_information_vec
                                    [*organism_number].x_location -
                                    all_spatial_biosphere_information.blob_vec
                                        [colony_blob_number].center_of_mass_x) +
                                (all_spatial_biosphere_information.organism_information_vec
                                    [*organism_number].y_location -
                                    all_spatial_biosphere_information.blob_vec
                                        [colony_blob_number].center_of_mass_y) *
                                    (all_spatial_biosphere_information.organism_information_vec
                                        [*organism_number].y_location -
                                        all_spatial_biosphere_information.blob_vec
                                            [colony_blob_number].center_of_mass_y)) *
                            all_spatial_biosphere_information.organism_information_vec
                                [*organism_number].mass;
                    }

                    all_spatial_biosphere_information.blob_vec[
                        colony_blob_number
                    ].blob_moment_of_inertia = moment_of_inertia;
                } else {
                    // Calculate the moment of inertia for oblong.
                    if
                        all_spatial_biosphere_information.organism_information_vec
                            [colony_blob_number].oblong
                    {
                        let mut moment_of_inertia = 0;

                        for other_circles in all_spatial_biosphere_information.organism_information_vec[
                            colony_blob_number
                        ].other_circle_positions.iter() {
                            moment_of_inertia =
                                (other_circles.distance_from_org_center *
                                    other_circles.distance_from_org_center *
                                    all_spatial_biosphere_information.organism_information_vec
                                        [colony_blob_number].mass) /
                                ((
                                    all_spatial_biosphere_information.organism_information_vec[
                                        colony_blob_number
                                    ].other_circle_positions.len() as i32
                                ) +
                                    1);
                        }
                        all_spatial_biosphere_information.blob_vec[
                            colony_blob_number
                        ].blob_moment_of_inertia = moment_of_inertia;

                        // Calculate the moment of inertia for a circle.
                    } else {
                        all_spatial_biosphere_information.blob_vec[
                            colony_blob_number
                        ].blob_moment_of_inertia =
                            (all_spatial_biosphere_information.organism_information_vec
                                [colony_blob_number].mass *
                                all_spatial_biosphere_information.organism_information_vec
                                    [colony_blob_number].radius *
                                all_spatial_biosphere_information.organism_information_vec
                                    [colony_blob_number].radius) /
                            2;
                    }
                }

                // This calculates the new x, y, and rotational velocities

                calculate_new_velocity(
                    all_spatial_biosphere_information,
                    &deterministic_trig,
                    colony_blob_number,
                    &original_blob
                );
            }
        }
    }
}

fn calculate_new_velocity(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    blob_number: usize,
    original_blob: &BlobRecord
) {
    // Give the new blob the x and y velocities of the old blob.
    all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity =
        original_blob.blob_x_velocity;
    all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity =
        original_blob.blob_y_velocity;

    // Then add on the angular velocity.
    // First maintain the rotation.
    all_spatial_biosphere_information.blob_vec[blob_number].angular_velocity =
        original_blob.angular_velocity;

    // Calculate the distance between the old blob center and the new blob center.
    let x_displacement =
        original_blob.center_of_mass_x -
        all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x;
    let y_displacement =
        original_blob.center_of_mass_x -
        all_spatial_biosphere_information.blob_vec[blob_number].center_of_mass_x;

    let distance_from_blob_center = square_root_64(
        (x_displacement as i64) * (x_displacement as i64) +
            (y_displacement as i64) * (y_displacement as i64)
    ) as i32;

    // Calculate the linear velocity from the distance and angular velocity.
    let linear_velocity = (distance_from_blob_center * original_blob.angular_velocity) / 1000;
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
    all_spatial_biosphere_information.blob_vec[blob_number].blob_x_velocity +=
        (linear_velocity * deterministic_trig.d_trig.cosine((angle_of_tangent, 1000)).0) / 1000;
    all_spatial_biosphere_information.blob_vec[blob_number].blob_y_velocity +=
        (linear_velocity * deterministic_trig.d_trig.sine((angle_of_tangent, 1000)).0) / 1000;
}
