use crate::simulation::AllSpatialBiosphereInformation;
use super::ColliderCircleInfo;

pub fn make_collider_circle_list(
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    blob_number: usize
) -> Vec<ColliderCircleInfo> {

    // This vec holds all the collider circles.
    let mut collider_circles: Vec<ColliderCircleInfo> = Vec::new();

    // This iterates over the blob to record every collider circle.
    for organism_number in all_spatial_biosphere_information.blob_vec[
        blob_number
    ].blob_members.iter() {

        // This adds the primary circles.
        collider_circles.push(ColliderCircleInfo {
            x: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].x_location,
            y: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].y_location,
            radius: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].radius,
            distance_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].distance_from_center_of_mass,
            angle_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                [*organism_number].angle_to_center_of_mass,
        });

        // This adds secondary circles.
        if all_spatial_biosphere_information.organism_information_vec[*organism_number].oblong {
            for circle in all_spatial_biosphere_information.organism_information_vec[
                *organism_number
            ].other_circle_positions.iter() {
                collider_circles.push(ColliderCircleInfo {
                    x: circle.x,
                    y: circle.y,
                    radius: circle.radius,
                    distance_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                        [*organism_number].distance_from_center_of_mass,
                    angle_to_center_of_mass: all_spatial_biosphere_information.organism_information_vec
                        [*organism_number].angle_to_center_of_mass,
                });
            }
        }
    }

    return collider_circles;
}
