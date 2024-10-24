use crate::{
    settings::GameSettings,
    simulation::{
        biosphere_simulation::collision_calculations::{
            LARGE_GRID_CIRCLE_MAX_RADIUS,
            SMALL_GRID_CIRCLE_MAX_RADIUS,
            SMALL_GRID_SIZE,
            LARGE_GRID_SIZE,
        },
        AllSpatialBiosphereInformation,
        CircleEntityType,
    },
    utility_functions::deterministic_trigonometry::DeterministicTrig,
};
use super::{ CollideeCircleInfo, ColliderCircleInfo };

// This function makes a vec with all the possible collidee circles in it.

pub fn make_collidee_circle_list(
    collider_circle: &ColliderCircleInfo,
    blob_number: usize,
    game_settings: &GameSettings,
    deterministic_trig: &DeterministicTrig,
    x_move: i32,
    y_move: i32,
    r_move: i32,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation
) -> Vec<CollideeCircleInfo> {
    let mut collidee_circles: Vec<CollideeCircleInfo> = Vec::new();

    let collider_radius = collider_circle.radius;

    // This calculates the x_move and y_move and includes the effect of rotation if rotation and the translational movement go in the same direction.
    let total_x_move: i32;
    let total_y_move: i32;

    // If rotation in involved.
    if r_move != 0 {
        // If the x movement from the rotation is in the same direction as the x movement from the translational velocity.
        if
            x_move.signum() ==
            (
                deterministic_trig.d_trig.cosine((
                    collider_circle.angle_to_center_of_mass + r_move,
                    1000,
                )).0 -
                deterministic_trig.d_trig.cosine((collider_circle.angle_to_center_of_mass, 1000)).0
            ).signum()
        {
            // Then add the x movement from rotation to the total x move.
            total_x_move =
                x_move +
                (collider_circle.distance_to_center_of_mass *
                    deterministic_trig.d_trig.cosine((
                        collider_circle.angle_to_center_of_mass + r_move,
                        1000,
                    )).0 -
                    collider_circle.distance_to_center_of_mass *
                        deterministic_trig.d_trig.cosine((
                            collider_circle.angle_to_center_of_mass,
                            1000,
                        )).0) /
                    1000;
        } else {
            // In this case where they go in different directions, if the x_move from the translational velocity is greater.
            if
                x_move.abs() >
                (
                    (collider_circle.distance_to_center_of_mass *
                        deterministic_trig.d_trig.cosine((
                            collider_circle.angle_to_center_of_mass + r_move,
                            1000,
                        )).0 -
                        collider_circle.distance_to_center_of_mass *
                            deterministic_trig.d_trig.cosine((
                                collider_circle.angle_to_center_of_mass,
                                1000,
                            )).0) /
                    1000
                ).abs()
            {
                // Then the total x_move is just the translational part of the x_move.
                total_x_move = x_move;
                // Otherwise if the rotational part is bigger, then the total x_move is the rotational part of the x_move
            } else {
                total_x_move =
                    (collider_circle.distance_to_center_of_mass *
                        deterministic_trig.d_trig.cosine((
                            collider_circle.angle_to_center_of_mass + r_move,
                            1000,
                        )).0 -
                        collider_circle.distance_to_center_of_mass *
                            deterministic_trig.d_trig.cosine((
                                collider_circle.angle_to_center_of_mass,
                                1000,
                            )).0) /
                    1000;
            }
        }

        // If the y movement from the rotation is in the same direction as the y movement from the translational velocity.
        if
            y_move.signum() ==
            (
                deterministic_trig.d_trig.sine((
                    collider_circle.angle_to_center_of_mass + r_move,
                    1000,
                )).0 -
                deterministic_trig.d_trig.sine((collider_circle.angle_to_center_of_mass, 1000)).0
            ).signum()
        {
            // Then add the y movement from rotation to the total y move.
            total_y_move =
                y_move +
                (collider_circle.distance_to_center_of_mass *
                    deterministic_trig.d_trig.sine((
                        collider_circle.angle_to_center_of_mass + r_move,
                        1000,
                    )).0 -
                    collider_circle.distance_to_center_of_mass *
                        deterministic_trig.d_trig.sine((
                            collider_circle.angle_to_center_of_mass,
                            1000,
                        )).0) /
                    1000;
        } else {
            // In this case where they go in different directions, if the y_move from the translational velocity is greater.
            if
                y_move.abs() >
                (
                    (collider_circle.distance_to_center_of_mass *
                        deterministic_trig.d_trig.sine((
                            collider_circle.angle_to_center_of_mass + r_move,
                            1000,
                        )).0 -
                        collider_circle.distance_to_center_of_mass *
                            deterministic_trig.d_trig.sine((
                                collider_circle.angle_to_center_of_mass,
                                1000,
                            )).0) /
                    1000
                ).abs()
            {
                // Then the total y_move is just the translational part of the y_move.
                total_y_move = y_move;
                // Otherwise if the rotational part is bigger, then the total y_move is the rotational part of the y_move
            } else {
                total_y_move =
                    (collider_circle.distance_to_center_of_mass *
                        deterministic_trig.d_trig.sine((
                            collider_circle.angle_to_center_of_mass + r_move,
                            1000,
                        )).0 -
                        collider_circle.distance_to_center_of_mass *
                            deterministic_trig.d_trig.sine((
                                collider_circle.angle_to_center_of_mass,
                                1000,
                            )).0) /
                    1000;
            }
        }
    } else {
        // If there is no rotations, then the translational portions are the only parts that need to be considered.
        total_x_move = x_move;
        total_y_move = y_move;
    }

    // This calculates the maximum and minimum x and y indexes for the small grid.
    let x_index_max = if
        (
            ((collider_circle.x +
                (if total_x_move > 0 { total_x_move } else { 0 }) +
                collider_radius +
                SMALL_GRID_CIRCLE_MAX_RADIUS +
                game_settings.map_width / 2) /
                SMALL_GRID_SIZE) as usize
        ) < all_spatial_biosphere_information.collision_detection_grid_small.len()
    {
        ((collider_circle.x +
            (if total_x_move > 0 { total_x_move } else { 0 }) +
            collider_radius +
            SMALL_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_width / 2) /
            SMALL_GRID_SIZE) as usize
    } else {
        all_spatial_biosphere_information.collision_detection_grid_small.len() - 1
    };

    let y_index_max = if
        (
            ((collider_circle.y +
                (if total_y_move > 0 { total_y_move } else { 0 }) +
                collider_radius +
                SMALL_GRID_CIRCLE_MAX_RADIUS +
                game_settings.map_height / 2) /
                SMALL_GRID_SIZE) as usize
        ) < all_spatial_biosphere_information.collision_detection_grid_small[0].len()
    {
        ((collider_circle.y +
            (if total_y_move > 0 { total_y_move } else { 0 }) +
            collider_radius +
            SMALL_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_height / 2) /
            SMALL_GRID_SIZE) as usize
    } else {
        all_spatial_biosphere_information.collision_detection_grid_small[0].len() - 1
    };

    let x_index_min = if
        (collider_circle.x +
            (if total_x_move < 0 { total_x_move } else { 0 }) -
            collider_radius -
            SMALL_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_width / 2) /
            SMALL_GRID_SIZE > 0
    {
        ((collider_circle.x +
            (if total_x_move < 0 { total_x_move } else { 0 }) -
            collider_radius -
            SMALL_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_width / 2) /
            SMALL_GRID_SIZE) as usize
    } else {
        0
    };

    let y_index_min = if
        (collider_circle.y +
            (if total_y_move < 0 { total_y_move } else { 0 }) -
            collider_radius -
            SMALL_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_height / 2) /
            SMALL_GRID_SIZE > 0
    {
        ((collider_circle.y +
            (if total_y_move < 0 { total_y_move } else { 0 }) -
            collider_radius -
            SMALL_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_height / 2) /
            SMALL_GRID_SIZE) as usize
    } else {
        0
    };

    // This adds all small collidees circles within the min and max grid square ranges to the collidee list.

    for x_index in x_index_min..=x_index_max {
        for y_index in y_index_min..=y_index_max {
            add_circles_in_small_grid(
                x_index,
                y_index,
                blob_number,
                &all_spatial_biosphere_information,
                &mut collidee_circles
            );
        }
    }

    // This calculates the maximum and minimum x and y indexes for the large grid.
    let x_index_max = if
        (
            ((collider_circle.x +
                (if total_x_move > 0 { total_x_move } else { 0 }) +
                collider_radius +
                LARGE_GRID_CIRCLE_MAX_RADIUS +
                game_settings.map_width / 2) /
                LARGE_GRID_SIZE) as usize
        ) < all_spatial_biosphere_information.collision_detection_grid_large.len()
    {
        ((collider_circle.x +
            (if total_x_move > 0 { total_x_move } else { 0 }) +
            collider_radius +
            LARGE_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_width / 2) /
            LARGE_GRID_SIZE) as usize
    } else {
        all_spatial_biosphere_information.collision_detection_grid_large.len() - 1
    };

    let y_index_max = if
        (
            ((collider_circle.y +
                (if total_y_move > 0 { total_y_move } else { 0 }) +
                collider_radius +
                LARGE_GRID_CIRCLE_MAX_RADIUS +
                game_settings.map_height / 2) /
                LARGE_GRID_SIZE) as usize
        ) < all_spatial_biosphere_information.collision_detection_grid_large[0].len()
    {
        ((collider_circle.y +
            (if total_y_move > 0 { total_y_move } else { 0 }) +
            collider_radius +
            LARGE_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_height / 2) /
            LARGE_GRID_SIZE) as usize
    } else {
        all_spatial_biosphere_information.collision_detection_grid_large[0].len() - 1
    };

    let x_index_min = if
        (collider_circle.x +
            (if total_x_move < 0 { total_x_move } else { 0 }) -
            collider_radius -
            LARGE_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_width / 2) /
            LARGE_GRID_SIZE > 0
    {
        ((collider_circle.x +
            (if total_x_move < 0 { total_x_move } else { 0 }) -
            collider_radius -
            LARGE_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_width / 2) /
            LARGE_GRID_SIZE) as usize
    } else {
        0
    };

    let y_index_min = if
        (collider_circle.y +
            (if total_y_move < 0 { total_y_move } else { 0 }) -
            collider_radius -
            LARGE_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_height / 2) /
            LARGE_GRID_SIZE > 0
    {
        ((collider_circle.y +
            (if total_y_move < 0 { total_y_move } else { 0 }) -
            collider_radius -
            LARGE_GRID_CIRCLE_MAX_RADIUS +
            game_settings.map_height / 2) /
            LARGE_GRID_SIZE) as usize
    } else {
        0
    };

    // This adds all large collidees circles within the min and max grid square ranges to the collidee list.

    for x_index in x_index_min..=x_index_max {
        for y_index in y_index_min..=y_index_max {
            add_circles_in_large_grid(
                x_index,
                y_index,
                blob_number,
                &all_spatial_biosphere_information,
                &mut collidee_circles
            );
        }
    }

    return collidee_circles;
}

fn add_circles_in_small_grid(
    x_index: usize,
    y_index: usize,
    blob_number: usize,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    collidee_circles: &mut Vec<CollideeCircleInfo>
) {
    // For every circle that is in the grid square
    for circle in all_spatial_biosphere_information.collision_detection_grid_small[x_index][
        y_index
    ].iter() {
        // If the circle is either a mineral or it is from a different blob.
        if
            circle.circle_entity_type == CircleEntityType::Mineral ||
            all_spatial_biosphere_information.organism_information_vec
                [circle.identity_number].blob_number != blob_number
        {
            // Then add it to the collidee list.
            if circle.circle_entity_type == CircleEntityType::Mineral {
                collidee_circles.push(CollideeCircleInfo {
                    x: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].x_location,
                    y: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].y_location,
                    radius: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].radius,
                    blob_number: 0,
                    circle_entity_type: CircleEntityType::Mineral,
                });
            } else {
                collidee_circles.push(CollideeCircleInfo {
                    x: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].x_location
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].x
                    },
                    y: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].y_location
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].y
                    },
                    radius: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].radius
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].radius
                    },
                    blob_number: all_spatial_biosphere_information.organism_information_vec
                        [circle.identity_number].blob_number,
                    circle_entity_type: CircleEntityType::Organism,
                });
            }
        }
    }
}

fn add_circles_in_large_grid(
    x_index: usize,
    y_index: usize,
    blob_number: usize,
    all_spatial_biosphere_information: &AllSpatialBiosphereInformation,
    collidee_circles: &mut Vec<CollideeCircleInfo>
) {
    // For every circle that is in the grid square
    for circle in all_spatial_biosphere_information.collision_detection_grid_large[x_index][
        y_index
    ].iter() {
        // If the circle is either a mineral or it is from a different blob.
        if
            circle.circle_entity_type == CircleEntityType::Mineral ||
            all_spatial_biosphere_information.organism_information_vec
                [circle.identity_number].blob_number != blob_number
        {
            // Then add it to the collidee list.
            if circle.circle_entity_type == CircleEntityType::Mineral {
                collidee_circles.push(CollideeCircleInfo {
                    x: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].x_location,
                    y: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].y_location,
                    radius: all_spatial_biosphere_information.mineral_information_vec
                        [circle.identity_number].radius,
                    blob_number: 0,
                    circle_entity_type: CircleEntityType::Mineral,
                });
            } else {
                collidee_circles.push(CollideeCircleInfo {
                    x: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].x_location
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].x
                    },
                    y: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].y_location
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].y
                    },
                    radius: if circle.circle_number == 0 {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].radius
                    } else {
                        all_spatial_biosphere_information.organism_information_vec
                            [circle.identity_number].other_circle_positions
                            [circle.circle_number - 1].radius
                    },
                    blob_number: all_spatial_biosphere_information.organism_information_vec
                        [circle.identity_number].blob_number,
                    circle_entity_type: CircleEntityType::Organism,
                });
            }
        }
    }
}
