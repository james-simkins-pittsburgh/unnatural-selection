use crate::simulation::AllBiosphereInformation;
use crate::simulation::AllSpeciesInformation;
use crate::simulation::AllMapInformation;
use crate::simulation::AllCurrentInformation;
use crate::simulation::AdministrativeInformation;
use crate::simulation::CheapRandomGameworld;
use crate::utility_functions::deterministic_trigonometry::DeterministicTrig;

pub fn simulate_biosphere(
    mut all_biosphere_information: &AllBiosphereInformation,
    all_species_information: &AllSpeciesInformation,
    all_map_information: &AllMapInformation,
    all_current_info: &AllCurrentInformation,
    admin_info: &AdministrativeInformation,
    mut cheap_random: &mut CheapRandomGameworld,
    d_trig: &DeterministicTrig
) {


}
