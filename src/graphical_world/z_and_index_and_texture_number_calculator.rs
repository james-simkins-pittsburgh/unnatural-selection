/* This module exists solely to house a function that sets the Z coordinate and index
of an entity. The Z coordinate is used primarily to ensure that textures
are not excessively loaded though it does also allow true depth in terms of 
the background and foreground status of organism */

pub fn calculate_z_and_index_and_texture_number(
    _animation_type: crate::simulation::AnimationType,
    _species_type: crate::simulation::SpeciesType,
    _background: bool
) -> (f32, usize, usize) {
    // This is a placeholder function
    // First return is the Z value, second is the index, third is the texture atlas number.
    return (0.0, 0, 0);
}
