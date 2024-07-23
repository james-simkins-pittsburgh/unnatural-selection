use bevy::prelude::*;

pub mod simulation_stepper;
pub mod current_simulation;
pub mod biosphere_simulation;

#[derive(Bundle, Default)]
pub struct GameworldBundle {
    pub all_biosphere_information: AllBiosphereInformation,
    pub all_current_information: AllCurrentInformation,
    pub all_map_information: AllMapInformation,
    pub all_species_information: AllSpeciesInformation,
}

// This contains all of the information about the simulation biosphere.
#[derive(Component, Default)]
pub struct AllBiosphereInformation {
    pub organism_information_vec: Vec<OrganismInformation>,
}

// This contains all the data about an individual organism.
#[derive(Copy, Clone, PartialEq, Default)]
pub struct OrganismInformation {
    pub in_use: bool,
    pub x_location: i32,
    pub y_location: i32,
    pub rotation: i32,
    pub health: i32,
    pub energy: i32,
    pub player_number: i32,
    pub species_number: i32,
    pub attached: bool,
    pub background: bool,
    pub no_collision_time_remaining: i32,
    pub main_animation_type: AnimationType,
    pub species_type: SpeciesType,
    pub moving_on_its_own: bool,
    pub eating: bool,
    pub eating_target: [usize; 3],
    pub in_host: bool,
    pub attachment_host: usize,
    pub inserting: bool,
    pub animation_counter: usize,
}

#[derive(Copy, Clone, PartialEq, Default)]
pub enum AnimationType {

    #[default] None,    
    Attacking,
    Growing,
    Lysing,
    Fission,
    Budding,

}

// This contains all the information about currents.
#[derive(Component, Default)]
pub struct AllCurrentInformation {
    pub current_information_vec: Vec<CurrentInformation>,
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct CurrentInformation {
    pub center_x: i32,
    pub center_y: i32,
    /* Measured from right x axis with counterclockwise positive.
`   Times 1000 to avoid floating point arithmetic.
    Ranges from minimum 0 to maximum 6282.
    This is the direction of acceleration. */
    pub angle_in_radians_times_1000: i32,
    // Measured in game distance unit per tick squared of acceleration produced.
    pub intensity: i32,
    // In game units
    pub radius: i32,
    // In ticks from start
    pub expiration_time: i32,
    // Is the current in the background?
    pub background: bool,
}

// This contains all the information about the map.
#[derive(Component, Default)]
pub struct AllMapInformation {
    /* The depth of the water every 10 game units staring from the right side.
measured in 10 game unit increments. */
    pub water_depth_in_10_unit_increments: Vec<i32>,
}

// This is the resource containing all species information
#[derive(Component, Default)]
pub struct AllSpeciesInformation {
    /* Information on all species is stored in this array. The first index is player number and the second
    index is species number */
    pub species_array: [[SingleSpeciesInformation; 8]; 16],
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct SingleSpeciesInformation {
    pub species_type: SpeciesType,
    pub species_role: SpeciesRole,
    pub species_domain: SpeciesDomain,
    pub species_genome: SpeciesGenome,
    pub species_characteristics: SpeciesCharacteristics,
}

/*  This enum specifies the type the species represents out of 20 categories roughly 
corresponding to phyla, families, or genuses of microorganism */
#[derive(Copy, Clone, PartialEq, Default)]
pub enum SpeciesType {
    #[default] Empty,
    // Consumers
    Apusozoa,
    Radiolaria,
    Paramecium,
    Euglena,
    Foraminfera,
    // Producers
    Prochlorococcus,
    Nostocales,
    Coccolithophore,
    Prediastrum,
    Prorocentrales,
    // Recyclers
    Fusarium,
    Rhodotorula,
    Vibrionales,
    Roseobacteria,
    Pelagibacteria,
    // Viruses
    Muvirus,
    Cystovirus,
    Mycovirus,
    Coccolithovirus,
    Phaeovirus,
}

// This enum specifies the ecological role of the species.
#[derive(Copy, Clone, PartialEq, Default)]
pub enum SpeciesRole {
    #[default] Empty,
    Consumer,
    Producer,
    Recycler,
    Virus,
}

// This enum specifies the domain of life of which the species is part.
#[derive(Copy, Clone, PartialEq, Default)]
pub enum SpeciesDomain {
    #[default] Empty,
    Protozoan,
    Fungus,
    Bacteria,
    Virus,
}

// This specifies whether or not each gene is turned on.
#[derive(Copy, Clone, PartialEq, Default)]
pub struct SpeciesGenome {
    // Many more to come!
    pub efficient_photosynthesis: bool,
    pub respiration: bool,
    pub cell_wall: bool,
}

// This specifies species characteristics and changes each time a gene changes
#[derive(Copy, Clone, PartialEq, Default)]
pub struct SpeciesCharacteristics {
    // Many more to come!
    // In health units per 15 ticks
    pub health_rate: i32,
    // In energy units per 15 ticks
    pub production_rate: i32,
    // Does it move on its own?
    pub motile: bool,
    // In nanometers per tick per tick
    pub movement_acceleration: i32,
    // Can it eat other organism?
    pub predator: bool,
    // Can it eat other motile protists?
    pub apex_predator: bool,
    // How much damage does it do per 15 ticks?
    pub attack_strength: i32,
    // How much energy the attack takes in per 15 ticks?
    pub attack_absorption: i32,
    // How far it can attack from?
    pub attack_range: i32,
    // How much does the organism weigh in picograms?
    pub mass: i32,
}
