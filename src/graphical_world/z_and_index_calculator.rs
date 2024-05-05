/* This module exists solely to house a function that sets the Z coordinate and index
of an entity. The Z coordinate is used primarily to ensure that textures
are not exessively loaded though it does also allow true depth in terms of 
the background and foreground status of organism */

pub fn calculate_z_and_index(
    animation_type: crate::simulation::AnimationType,
    species_type: crate::simulation::SpeciesType,
    background: bool,
) -> (f32, usize)

{

    // This is a placeholder.
    return (0.0,0)

}
