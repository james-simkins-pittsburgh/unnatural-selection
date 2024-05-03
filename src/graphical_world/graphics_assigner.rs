use bevy::prelude::*;

/* This module contains the systems that assign simulation organisms/viruses to graphical 
entities when they become visible, removes assignments when simulations become invisible, 
and created more graphical entities when they are invisible.*/

pub fn unassign_graphical_entities(
    mut need_unassigned_struct: ResMut<crate::graphical_world::OrganismsToUnboundFromGraphicalPartner>,
    mut assigned_graphical_entities: Query<
        (&mut crate::graphical_world::MainGraphicsOfOrganism, Entity),
        With<crate::graphical_world::Assigned>
    >,
    mut number_unbound: ResMut<crate::graphical_world::NumberOfUnboundOrganisms>,
    mut commands: Commands
) {
    // Checks each graphical entity to see if it needs to be unbound.
    for mut graphical_entity in assigned_graphical_entities.iter_mut() {
        // Checks to make sure there are still unbindings that need to occur.
        if need_unassigned_struct.organism_to_unbound_from_graphical_partner.len() == 0 {
            break;
        }

        /* Checks each item in the unbinding vec to see if it includes the organism corresponsing to 
        this particular graphical entity. */
        for index in 0..need_unassigned_struct.organism_to_unbound_from_graphical_partner.len() {
            // Checks to see if the corresponding organism matches the item in the vec.
            if
                need_unassigned_struct.organism_to_unbound_from_graphical_partner[index] ==
                graphical_entity.0.corresponsing_organism_number
            {
                // Sets the corresponding organism number to 0.
                graphical_entity.0.corresponsing_organism_number = 0;
                // Sets the unassigned marker component.
                commands
                    .entity(graphical_entity.1)
                    .remove::<crate::graphical_world::Assigned>()
                    .insert(crate::graphical_world::Unassigned);
                // Increases the number of unbound graphical entities by 1.
                number_unbound.number_unbound = number_unbound.number_unbound + 1;
                // Removes the unassigned organism number from the vec of organism that need to be unassigned.
                need_unassigned_struct.organism_to_unbound_from_graphical_partner.swap_remove(
                    index
                );
                // Breaks out of the for loop since unassignment has already occured.
                break;
            }
        }
    }
}

/* This adds a sufficient number of main graphical entities to have at least one for each simulation organism
that is visible to the camera. */
pub fn create_graphical_entities(
    need_assignment_struct: Res<crate::graphical_world::OrganismsThatNeedGraphicalPartner>,
    mut number_unbound: ResMut<crate::graphical_world::NumberOfUnboundOrganisms>,
    mut commands: Commands
) {
    if
        (need_assignment_struct.organism_that_need_graphical_partner.len() as i32) >
        number_unbound.number_unbound
    {
        for _n in 0..(need_assignment_struct.organism_that_need_graphical_partner.len() as i32) -
            number_unbound.number_unbound {
            commands.spawn((
                crate::graphical_world::MainGraphicsOfOrganism { ..Default::default() },
                crate::graphical_world::Unassigned,
            ));
        }
        number_unbound.number_unbound =
            need_assignment_struct.organism_that_need_graphical_partner.len() as i32;
    }
}

pub fn assign_graphical_entities(
    mut need_assignment_struct: ResMut<crate::graphical_world::OrganismsThatNeedGraphicalPartner>,
    mut unassigned_graphical_entities: Query<
        &mut crate::graphical_world::MainGraphicsOfOrganism,
        With<crate::graphical_world::Unassigned>
    >,
    mut number_unbound: ResMut<crate::graphical_world::NumberOfUnboundOrganisms>
) {

    for mut graphical_entity in unassigned_graphical_entities.iter_mut() {

        if need_assignment_struct.organism_that_need_graphical_partner.len() == 0 {
            break;
        }

        // This assigns the first simulation organism on the need assignment list to a graphical entity.
        graphical_entity.corresponsing_organism_number = need_assignment_struct.organism_that_need_graphical_partner [1];
        // This removes that organism now that it has been assigned.
        need_assignment_struct.organism_that_need_graphical_partner.swap_remove(1);
        // This subtracts one from the count of graphical entities that are unbounbd.
        number_unbound.number_unbound = number_unbound.number_unbound - 1;
    }

}
