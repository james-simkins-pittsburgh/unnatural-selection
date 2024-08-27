// This utility function calculates square root for integers.

pub fn square_root(input: i64) -> i64 {
    if input <= 0 {
        /* Negative input may occur due to rounding errors when the answer should be 0.
        Therefore, it makes sense to return 0 in that case */
        return 0;
    }

    // Application of Heron's method.

    let mut power_of_100: u32 = 1;
    let one_hundred: i64 = 100;
    let ten: i64 = 10;

    // Figure out the magnitude of the answer and make an estimate.
    while one_hundred.pow(power_of_100) < input {
        power_of_100 = power_of_100 + 1;
        println!("Help, I am stuck!");
    }

    let mut estimate;

    if input / one_hundred.pow(power_of_100 - 1) < 10 {
        estimate = 2 * ten.pow(power_of_100 - 1);
    } else {
        estimate = 6 * ten.pow(power_of_100 - 1);
    }

    // Repeat until the estimate is a better estimate that the integer above or below it.
    while
        (input - estimate * estimate).abs() <= (input - (estimate + 1) * (estimate + 1)).abs() &&
        (input - estimate * estimate).abs() <= (input - (estimate - 1) * (estimate - 1)).abs()
    {
        estimate = (estimate + input / estimate) / 2;
        println!("Help, I am stucker!");
        println!("{}", estimate);
    }

return estimate;

}

#[test]
fn test_square_root() {
    println!("Hi There!");
    assert_eq!(square_root(38), 6);
    assert_eq!(square_root(100), 10);
    assert_eq!(square_root(223), 15)
}
