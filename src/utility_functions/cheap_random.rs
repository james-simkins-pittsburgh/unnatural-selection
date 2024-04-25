use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/* This module exists to give something close enough to a random number generator that is deterministic 
but is also much cheaper than directly using a random number generator. It does this by generating a list of 
10,000 i16 numbers between 0 and 359. This range was chosen because 360 is divisible by 1,2,3,4,5,6,8,9,10,
and 12 which allows lots of random possibilities easily */


#[derive(Resource)]
pub struct Random0To355ForBiosphere {

    array_random_0_to_359: [i16; 10000],
    last_index_used_0_to_359: usize,

}


#[derive(Resource)]
pub struct Random0To355ForCurrents {

    array_random_0_to_359: [i16; 10000],
    last_index_used_0_to_359: usize,

}

fn generate_random_0_359 (seed: u64, rand_num_gen: ChaCha8Rng)-> [i16; 10000] {


}

fn next_random_bio (random_0_to_355: &mut Random0To355ForBiosphere) {

    

}

fn next_random_curr (random_0_to_355: &mut Random0To355ForCurrent) {


}