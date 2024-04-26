use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/* This module exists to give something close enough to a random number generator that is deterministic 
and portable but is also much cheaper than directly using a random number generator. It does this by 
generating a list of 10,000 i16 numbers between 0 and 359. This range was chosen because 360 is divisible 
by 1,2,3,4,5,6,8,9,10, and 12 which allows lots of random possibilities easily */

pub struct Random0to359 {
    pub array: [i16; 10000],
    pub next_index: usize,
}

#[derive(Resource)]
pub struct RandomForBiosphere {
    pub random_0_to_359: Random0to359,
}

#[derive(Resource)]
pub struct RandomForCurrents {
    pub random_0_to_359: Random0to359,
}

#[derive(Resource)]
pub struct RandomForComputerPlayers {
    pub random_0_to_359: Random0to359,
}

pub fn generate_random_0_359(seed: u64) -> Random0to359 {

    let mut prng_from_seed = ChaCha8Rng::seed_from_u64(seed);

    let mut random_array: [i16; 10000] = [0; 10000];

    for index in 0..10000 {
        random_array[index] = (prng_from_seed.next_u64() % 360) as i16;
    }

    return Random0to359 { array: random_array, next_index: 0 };
}

pub fn next_random(random_0_to_359: &mut Random0to359) -> i16 {

    if random_0_to_359.next_index <99999 {
        
    random_0_to_359.next_index = random_0_to_359.next_index + 1;

    return random_0_to_359.array [random_0_to_359.next_index -1]

    } else {

        random_0_to_359.next_index = 0;
    
        return random_0_to_359.array [100000]

    }
   
}