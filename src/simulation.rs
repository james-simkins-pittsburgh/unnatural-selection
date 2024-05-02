use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct GameworldBundle {
    pub all_biosphere_information: AllBiosphereInformation,
    pub all_current_information: AllCurrentInformation,
    pub all_map_information: AllMapInformation,
    pub all_species_information: AllSpeciesInformation,
}

// This contains all of the information about the simulation bisphere.
#[derive(Component, Default)]
pub struct AllBiosphereInformation {
    pub organism_information_vec: Vec<OrganismInformation>,
    pub virus_information_vec: Vec<VirusInformation>,
}

// This contains all the data about an individual organism.
#[derive(Copy, Clone, PartialEq, Default)]
pub struct OrganismInformation {
    pub in_use: bool,
    pub x_location: i32,
    pub y_location: i32,
    pub health: i16,
    pub energy: i16,
    pub player_number: i8,
    pub species_number: i8,
    pub attached: bool,
    pub background: bool,
    pub no_collision_time_remaining: i8,
    pub animation_type: AnimationType,
    pub animation_counter: i8,
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


// This contains all the data about an viruses.
#[derive(Copy, Clone, PartialEq, Default)]
pub struct VirusInformation {
    pub x_location: i32,
    pub y_location: i32,
    pub player_number: i8,
    pub species_number: i8,
    pub in_host: bool,
}

// This contains all the information about currents.
#[derive(Component, Default)]
pub struct AllCurrentInformation {
    pub current_information_vec: Vec<CurrentInformation>,
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct CurrentInformation {
    pub bottom_left_x: i32,
    pub bottom_left_y: i32,
    /* Measured from right x axis with counterclockwise positive.
`   Times 100 to avoid floating point arithmentic.
    Ranges from minimum 0 to maxiumum 627.
    This is the direction of acceleration. */
    pub angle_in_radians_times_100: i32,
    // Measured in game distance unit per tick squared of acceleration produced.
    pub intensity: i32,
    // In game units
    pub height: i32,
    // In game units
    pub width: i32,
    // In ticks from start
    pub expiration_time: i32,
}

// This contains all the information about the map.
#[derive(Component, Default)]
pub struct AllMapInformation {
    /* The depth of the water every 10 game units staring from the right side.
measures in 10 game unit increments. */
    pub water_depth_in_10_unit_increments: [i16; crate::MAP_WIDTH / 1000],
}

// This is the resource containing all species information
#[derive(Component, Default)]
pub struct AllSpeciesInformation {
    /* Information on all species is stored in this array. The first index is player number and the second
    index is species number */
    pub species_array: [[SingleSpeciesInformation; 6]; 8],
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
corresponding to phylums, families, or genuses of microrganism */
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
    // Recylers
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

// This enumn specifies the domain of life of which the species is part.
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
    pub health_rate: u8,
    // In energy unites per 15 ticks
    pub production_rate: u8,
    // Does it move on its own?
    pub motile: bool,
    // In nanometers per tick per tick
    pub movement_acceleration: u8,
    // Can it eat other organism?
    pub predator: bool,
    // Can it eat other motile protists?
    pub apex_predator: bool,
    // How much damage does it do per 15 ticks?
    pub attack_stength: u8,
    // How much energy the attack takes in per 15 ticks?
    pub attack_absorbtion: u8,
    // How far it can attack from
    pub attack_range: u8,
    // How much does the organism weigh in picograms?
    pub mass: u8,
}
