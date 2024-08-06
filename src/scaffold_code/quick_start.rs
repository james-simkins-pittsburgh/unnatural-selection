use bevy::prelude::*;

pub fn create_basic_world(mut commands: Commands) {
    commands.spawn(crate::simulation::GameworldBundle { ..Default::default() });
}

pub fn populate_basic_world(
    mut gameworld_query: Query<(&mut crate::simulation::AllBiosphereInformation, &mut crate::simulation::CheapRandomGameworld)>
) {
    let (mut biosphere, mut cheap_random_gameworld) = gameworld_query.single_mut();
    cheap_random_gameworld.random_0_to_359 = crate::utility_functions::cheap_random::Random0to359::initialize(412);
    biosphere.organism_information_vec = Vec::new();

    let mut x_location = 0;
    let mut y_location = 0;

    biosphere.organism_information_vec.push(crate::simulation::OrganismInformation {
        in_use: false,
        x_location: 0,
        y_location: 0,
        x_velocity: 0,
        y_velocity: 0,
        rotation: 0,
        health: 0,
        energy: 0,
        player_number: 0,
        species_number: 0,
        species_type: crate::simulation::SpeciesType::Empty,
        background: false,
        animation_type: crate::simulation::AnimationType::None,
        moving_on_its_own: false,
        eating: false,
        eating_target: Vec::new(),
        in_host: false,
        inserting: false,
        viral_host_organism: 0,
        animation_counter: 0,
        blob_number: 0,
        blob_attached_entities: Vec::new(),
        colony_number: 0,
        colony_attached_entities: Vec::new(),
        attached_to_host: false,
        part_of_multi_org_blob: false,
    });

    biosphere.blob_vec.push(crate::simulation::BlobRecord {
        in_use: false,
        blob_members: Vec::new(),
        blob_x_velocity: 0,
        blob_y_velocity: 0,
        blob_mass: 0,
    });

    for org_num in 1..21 {
        match org_num {
            1 => {
                x_location = -5000;
                y_location = -10000;
            }
            2 => {
                x_location = -5000;
                y_location = -5000;
            }
            3 => {
                x_location = -5000;
                y_location = 0;
            }
            4 => {
                x_location = -5000;
                y_location = 5000;
            }
            5 => {
                x_location = -5000;
                y_location = 10000;
            }
            6 => {
                x_location = -5000;
                y_location = 15000;
            }
            7 => {
                x_location = 0;
                y_location = -15000;
            }
            8 => {
                x_location = 0;
                y_location = -10000;
            }
            9 => {
                x_location = 0;
                y_location = -5000;
            }
            10 => {
                x_location = 0;
                y_location = 0;
            }
            11 => {
                x_location = 0;
                y_location = 5000;
            }
            12 => {
                x_location = 0;
                y_location = 10000;
            }
            13 => {
                x_location = 0;
                y_location = 15000;
            }
            14 => {
                x_location = 5000;
                y_location = -15000;
            }
            15 => {
                x_location = 5000;
                y_location = -10000;
            }
            16 => {
                x_location = 5000;
                y_location = -5000;
            }
            17 => {
                x_location = 5000;
                y_location = 0;
            }
            18 => {
                x_location = 5000;
                y_location = 5000;
            }
            19 => {
                x_location = 5000;
                y_location = 10000;
            }
            20 => {
                x_location = -5000;
                y_location = -15000;
            }
            _ => {}
        }

        biosphere.organism_information_vec.push(crate::simulation::OrganismInformation {
            in_use: true,
            x_location: x_location,
            y_location: y_location,
            x_velocity: 0,
            y_velocity: 0,
            rotation: 0,
            health: 100,
            energy: 100,
            player_number: 1,
            species_number: 1,
            species_type: crate::simulation::SpeciesType::Prochlorococcus,
            background: false,
            animation_type: crate::simulation::AnimationType::None,
            moving_on_its_own: false,
            eating: false,
            eating_target: Vec::new(),
            in_host: false,
            inserting: false,
            viral_host_organism: 0,
            animation_counter: 0,
            blob_number: org_num,
            blob_attached_entities: Vec::new(),
            colony_number: 0,
            colony_attached_entities: Vec::new(),
            attached_to_host: false,
            part_of_multi_org_blob: false,
        });

        biosphere.blob_vec.push(crate::simulation::BlobRecord {
            in_use: true,
            blob_members: vec![org_num],
            blob_x_velocity: 0,
            blob_y_velocity: 0,
            blob_mass: 1,
        });
    }
}

pub fn add_to_graphics(
    mut need_partner_list: ResMut<crate::graphical_world::OrganismsThatNeedGraphicalPartner>,
) {
    need_partner_list.organism_that_need_graphical_partner = Vec::new();

    for index in 1..21 {
        need_partner_list.organism_that_need_graphical_partner.push(index)
    }

}
