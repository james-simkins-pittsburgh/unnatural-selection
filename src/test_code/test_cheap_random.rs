use crate::utility_functions::cheap_random;

pub fn test_cheap_random () {
    let mut test_cheap_random = cheap_random::RandomForBiosphere{
        random_0_to_359: cheap_random::generate_random_0_359(2),
    };

    for i in 0..10 {
        println! ("{}", cheap_random::next_random(&mut test_cheap_random.random_0_to_359));


    }
}
