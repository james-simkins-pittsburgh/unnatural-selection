use bevy::prelude::*;

/* This module contains the systems that assign simulation organisms/viruses to graphical 
entities when they become visible, removes assignments when simulations become invisible, 
and created more graphical entities when they are invisible.*/

pub fn unassign_graphical_entities(
    need_unassigned_struct : ResMut<crate::graphical_world::OrganismsToUnboundFromGraphicalPartner>,
    assigned_graphical_entities: Query<(&mut crate::graphical_world::MainGraphicsOfOrganism, &mut crate::graphical_world::Assigned)>,
    number_unbound: ResMut<crate::graphical_world::NumberOfUnboundOrganisms>
) {}


pub fn create_graphical_entities(
    need_assignment_struct: ResMut<crate::graphical_world::OrganismsThatNeedGraphicalPartner>,
    number_unbound: ResMut<crate::graphical_world::NumberOfUnboundOrganisms>,

) {}

pub fn assign_graphical_entities(
    need_assignment_struct: ResMut<crate::graphical_world::OrganismsThatNeedGraphicalPartner>, 
    unassigned_graphical_entities: Query<(&mut crate::graphical_world::MainGraphicsOfOrganism, &mut crate::graphical_world::Unassigned)>,
    number_unbound: ResMut<crate::graphical_world::NumberOfUnboundOrganisms>,

) {}
