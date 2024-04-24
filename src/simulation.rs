use bevy::prelude::*;

// This contains all of the information about the simulation bisphere.
#[derive(Component)]
pub struct BiosphereInformation {
   organism_information_vec: Vec<OrganismInformation>,

}

// This contains all the data about an individual organism.
pub struct OrganismInformation {

    x_location: i32,
    y_location: i32,
    health: i16,
    energy: i16,
    player_number: u8,
    species_number: u8,
    attached: bool,
    background: bool,
    no_collision_time_remaining: u8,
}

// This contains all the data about an viruses.
pub struct Virusnformation {

    x_location: i32,
    y_location: i32,
    player_number: u8,
    species_number: u8,
    in_host: bool,
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
    pub health_rate: u8,
    pub production_rate: u8,
    pub motile: bool,
    pub movement_speed: u8,
    pub predator: bool,
    pub attack_stength: u8,
    pub attack_range: u8,
    pub weight: u8,
}
