use crate::simulation::AllSpatialBiosphereInformation;

pub fn apply_collision(
    all_spatial_biosphere_information: &mut AllSpatialBiosphereInformation,
    combination_list: &Vec<usize>,
    _mineral_involved: bool,
) {

    for blob_number in combination_list.iter() {

        all_spatial_biosphere_information.blob_vec[*blob_number].blob_x_velocity = 0;
        all_spatial_biosphere_information.blob_vec[*blob_number].blob_y_velocity = 0;
        all_spatial_biosphere_information.blob_vec[*blob_number].angular_velocity = 0;
    
    }

}
