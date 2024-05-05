use bevy::prelude::*;

pub mod graphics_assigner;
pub mod graphics_updater;
pub mod texture_loader;

// When an organism that was not in the camera area is moves into camera it's index number is added to this vec.
#[derive(Resource)]
pub struct OrganismsThatNeedGraphicalPartner {
    pub organism_that_need_graphical_partner: Vec<usize>,
}

// When an organism that was in the camera area moves out of camera or is destroyes it is added to this vec.
#[derive(Resource)]
pub struct OrganismsToUnboundFromGraphicalPartner {
    pub organism_to_unbound_from_graphical_partner: Vec<usize>,
}

#[derive(Resource)]
pub struct NumberOfUnboundOrganisms {
    pub number_unbound: i32,
}

#[derive(Component, Default)]
pub struct MainGraphicsOfOrganism {
    pub x_location: i32,
    pub y_location: i32,
    pub corresponsing_organism_number: usize,
    pub animation_type: crate::simulation::AnimationType,
    pub animation_counter: i8,
    pub species_type: crate::simulation::SpeciesType,
    // If true, these mark different child entities that should be created.
    // Many more needed. These are examples.
    pub cell_wall: bool,
    pub flagella_1: bool,
    pub flagella_2: bool,
    pub cillia_1: bool,
    pub cillia_2: bool,
}

// This marks graphical entities with assignments.
#[derive(Component)]
pub struct Assigned;

// This marks graphical entities without assignments.
#[derive(Component)]
pub struct Unassigned;