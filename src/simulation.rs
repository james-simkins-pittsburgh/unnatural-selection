use bevy::prelude::*;

// This contains all of the information about the simulation bisphere.
#[derive(Component)]
pub struct BiosphereInformation {
    organism_information_vec: Vec<OrganismInformation>,
    virus_information_vec: Vec<VirusInformation>,
}

// This contains all the data about an individual organism.
pub struct OrganismInformation {
    x_location: i32,
    y_location: i32,
    health: i16,
    energy: i16,
    player_number: i8,
    species_number: i8,
    attached: bool,
    background: bool,
    no_collision_time_remaining: i8,
}

// This contains all the data about an viruses.
pub struct VirusInformation {
    x_location: i32,
    y_location: i32,
    player_number: i8,
    species_number: i8,
    in_host: bool,
}

// This contains all the information about currents.
#[derive(Component)]
pub struct AllCurrentInformation {
    current_information_vec: Vec<CurrentInformation>,
}

pub struct CurrentInformation {
    bottom_left_x: i32,
    bottom_left_y: i32,
    /* Measured from right x axis with counterclockwise positive.
`   Times 100 to avoid floating point arithmentic.
    Ranges from minimum 0 to maxiumum 627.
    This is the direction of acceleration. */
    angle_in_radians_times_100: i32,
    // Measured in game distance unit per tick squared of acceleration produced.
    intensity: i32,
    // In game units
    height: i32,
    // In game units
    width: i32,
    // In ticks from start
    expiration_time: i32,
}

// This contains all the information about the map.
#[derive(Component)]
pub struct MapInformation {
    /* The depth of the water every 10 game units staring from the right side.
measures in 10 game unit increments. */
    water_depth_in_10_unit_increments: [i16; crate::MAP_WIDTH / 1000],
}

// This is the resource containing all species information
#[derive(Resource, Default)]
pub struct AllSpeciesInformation {
    /* Information on all species is stored in this array. The first index is player number and the second
    index is species number */
    species_array: [[SingleSpeciesInformation; 6]; 8],
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
