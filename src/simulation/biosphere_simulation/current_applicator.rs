use crate::simulation::AllBiosphereInformation;
use crate::simulation::AllCurrentInformation;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

pub fn apply_current(
    all_biosphere_information: &mut AllBiosphereInformation,
    deterministic_trig: &DeterministicTrig,
    all_current_information: &AllCurrentInformation,
    blob_number: usize
) {
    let blob_x = all_biosphere_information.blob_vec[blob_number].center_of_mass_x;
    let blob_y = all_biosphere_information.blob_vec[blob_number].center_of_mass_y;
    let blob_mass = all_biosphere_information.blob_vec[blob_number].blob_mass;

    // For every organism in the blob
    for index in 0..all_biosphere_information.blob_vec.len() as usize {
        let organism_num = all_biosphere_information.blob_vec[blob_number].blob_members[index];
        let organism_x = all_biosphere_information.organism_information_vec
            [organism_num].x_location;
        let organism_y = all_biosphere_information.organism_information_vec
            [organism_num].y_location;
        let organism_m = all_biosphere_information.organism_information_vec[organism_num].mass;

        // For every current
        for current_number in 0..all_current_information.current_information_vec.len() as usize {
            let current_x = all_current_information.current_information_vec
                [current_number].center_x;
            let current_y = all_current_information.current_information_vec
                [current_number].center_y;
            let current_r = all_current_information.current_information_vec[current_number].radius;

            // If the organism is within circle that represents the current
            if
                (current_x - organism_x) * (current_x - organism_x) +
                    (current_y - organism_y) * (current_y - organism_y) <= current_r * current_r
            {
                // Then apply the current to the blob
                let current_a = all_current_information.current_information_vec
                    [current_number].angle_in_radians_times_1000;
                let current_i = all_current_information.current_information_vec
                    [current_number].intensity;

                if
                    all_biosphere_information.organism_information_vec
                        [organism_num].part_of_multi_org_blob
                {
                    // In the case of multi-organism blobs, mass and angle to blob center impact rotational dynamics.
                    let org_mass = all_biosphere_information.organism_information_vec [organism_num].mass;
                    let angle_to_blob_center = deterministic_trig.d_trig.arctangent((((blob_y-organism_y) *1000)/(blob_x-organism_x),1000)).0;
                    
                    // This changes the ordinary velocity for the blob based on the current
                    let velocity_component = deterministic_trig.d_trig.cosine(((current_a - angle_to_blob_center), 1000)).0*current_i/1000;
                    all_biosphere_information.blob_vec[blob_number].blob_x_velocity +=
                        (deterministic_trig.d_trig.cosine((angle_to_blob_center, 1000)).0 * velocity_component) * (org_mass/blob_mass) / 1000;
                    all_biosphere_information.blob_vec[blob_number].blob_y_velocity +=
                        (deterministic_trig.d_trig.sine((angle_to_blob_center, 1000)).0 * velocity_component) *(org_mass/blob_mass) / 1000;

                    // Left off here. Need to calculate moment of inertia before I can calculate angular velocity.
                    let angular_velocity_component = deterministic_trig.d_trig.sine(((current_a - angle_to_blob_center), 1000)).0*current_i/1000;
                        

                } else {
                    // The acceleration can be added directly if the blob is a single organism.
                    all_biosphere_information.blob_vec[blob_number].blob_x_velocity +=
                        (deterministic_trig.d_trig.cosine((current_a, 1000)).0 * current_i) / 1000;
                        all_biosphere_information.blob_vec[blob_number].blob_y_velocity +=
                        (deterministic_trig.d_trig.sine((current_a, 1000)).0 * current_i) / 1000;
                }
            }
        }
    }
}
