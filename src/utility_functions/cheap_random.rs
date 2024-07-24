use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/* This module exists to give something close enough to a random number generator that is deterministic 
and portable but is also much cheaper than directly using a random number generator. It does this by 
generating a list of 10,000 i32 numbers between 0 and 359. This range was chosen because 360 is divisible 
by 1,2,3,4,5,6,8,9,10, and 12 which allows lots of random possibilities easily */

pub struct Random0to359 {
    pub array: [i32; 10000],
    pub next_index: usize,
}

impl Random0to359 {
    pub fn initialize(seed: u64) -> Self {
        let mut prng_from_seed = ChaCha8Rng::seed_from_u64(seed);

        let mut random_array: [i32; 10000] = [0; 10000];

        for index in 0..10000 {
            random_array[index] = (prng_from_seed.next_u64() % 360) as i32;
        }

        return Random0to359 { array: random_array, next_index: 0 };
    }

    pub fn next_random(&mut self) -> i32 {
        if self.next_index < 99999 {
            self.next_index = self.next_index + 1;

            return self.array[self.next_index - 1];
        } else {
            self.next_index = 0;

            return self.array[100000];
        }
    }
}

impl Default for Random0to359 {
    fn default() -> Self {
        Random0to359 { array: [0; 10000], next_index: 0 }
    }
}