use crate::simulation::AllBiosphereInformation;

pub fn apply_collision(
    all_biosphere_information: &mut AllBiosphereInformation,
    combination_list: &Vec<usize>,
    mineral_involved: bool,
) {

    for blob_number in combination_list.iter() {

        all_biosphere_information.blob_vec[*blob_number].blob_x_velocity = 0;
        all_biosphere_information.blob_vec[*blob_number].blob_y_velocity = 0;
        all_biosphere_information.blob_vec[*blob_number].angular_velocity = 0;
    
    }

}
