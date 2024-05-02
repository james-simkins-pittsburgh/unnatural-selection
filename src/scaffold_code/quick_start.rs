use bevy::prelude::*;

pub fn create_basic_world(mut commands: Commands) {
    commands.spawn(crate::simulation::GameworldBundle { ..Default::default() });
}

pub fn populate_basic_world(
    mut biosphere_query: Query<&mut crate::simulation::AllBiosphereInformation>
) {
    for mut biosphere in biosphere_query.iter_mut() {
        biosphere.organism_information_vec = Vec::new();

        let mut x_location = 0;
        let mut y_location = 0;

        for org_num in 0..20 {
            match org_num {
                0 => {
                    x_location = 0;
                    y_location = -4000;
                }
                1 => {
                    x_location = 0;
                    y_location = -3000;
                }
                2 => {
                    x_location = 0;
                    y_location = -2000;
                }
                3 => {
                    x_location = 0;
                    y_location = -1000;
                }
                4 => {
                    x_location = 0;
                    y_location = 0;
                }
                5 => {
                    x_location = 0;
                    y_location = 1000;
                }
                6 => {
                    x_location = 0;
                    y_location = 2000;
                }
                7 => {
                    x_location = 0;
                    y_location = 3000;
                }
                8 => {
                    x_location = 0;
                    y_location = 4000;
                }
                9 => {
                    x_location = 0;
                    y_location = 5000;
                }
                10 => {
                    x_location = 1000;
                    y_location = -4000;
                }
                11 => {
                    x_location = 1000;
                    y_location = -3000;
                }
                12 => {
                    x_location = 1000;
                    y_location = -2000;
                }
                13 => {
                    x_location = 1000;
                    y_location = -1000;
                }
                14 => {
                    x_location = 1000;
                    y_location = 0;
                }
                15 => {
                    x_location = 1000;
                    y_location = 1000;
                }
                16 => {
                    x_location = 1000;
                    y_location = 2000;
                }
                17 => {
                    x_location = 1000;
                    y_location = 3000;
                }
                18 => {
                    x_location = 1000;
                    y_location = 4000;
                }
                19 => {
                    x_location = 1000;
                    y_location = 5000;
                }
                _ => {}
            }

            biosphere.organism_information_vec.push(crate::simulation::OrganismInformation {
                in_use: true,
                x_location: x_location,
                y_location: y_location,
                health: 100,
                energy: 100,
                player_number: 1,
                species_number: 1,
                attached: false,
                background: false,
                no_collision_time_remaining: 0,
                animation_type: crate::simulation::AnimationType::None,
                animation_counter: 0,
                in_host: false,
            });
        }
    }
}

pub fn add_to_graphics(
    mut need_partner_list: ResMut<crate::graphical_world::OrganismsThatNeedGraphicalPartner>
) {
    for index in 0..20 {
        need_partner_list.organism_that_need_graphical_partner.push(index);
    }
}
