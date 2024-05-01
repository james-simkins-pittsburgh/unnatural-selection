use bevy::prelude::*;

// When an organism that was not in the camera area is created 
#[derive(Resource)] 
pub struct OrganismsThatNeedGraphicalPartner {

pub organism_that_need_graphical_partner: Vec<usize>

}

#[derive(Resource)] 
pub struct OrganismsToUnboundFromGraphicalPartner {

pub organism_that_need_graphical_partner: Vec<usize>

}