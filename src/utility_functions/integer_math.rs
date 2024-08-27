// This utility function calculates square root for integers.

pub fn square_root(input: i64) -> i64 {
    if input <= 0 {
        /* Negative input may occur due to rounding errors when the answer should be 0.
        Therefore, it makes sense to return 0 in that case */
        return 0;
    }

    // Application of Heron's method.

    let mut power_of_100: u32 = 1;
    // Figure out the magnitude of the answer and make an estimate.
    while i64::pow(100i64, power_of_100) < input {
        power_of_100 = power_of_100 + 1;
    }

    let mut estimate;
    let mut previous_estimate = -1;

    if input / i64::pow(100i64, power_of_100 - 1) < 10 {
        estimate = 2 * i64::pow(10i64, power_of_100 - 1);
    } else {
        estimate = 6 * i64::pow(10i64, power_of_100 - 1);
    }

    // Repeat until the estimate is a better estimate that the integer above or below it.
    while
        (input - estimate * estimate).abs() > (input - (estimate + 1) * (estimate + 1)).abs() ||
        (input - estimate * estimate).abs() > (input - (estimate - 1) * (estimate - 1)).abs()
    {
        estimate = (estimate + input / estimate) / 2;
        if estimate == previous_estimate {
            // This extra code is needed because it gets stuck if the answer is closer to one larger because of integer division.
            if
                (input - estimate * estimate).abs() >
                (input - (estimate + 1) * (estimate + 1)).abs()
            {
                return estimate + 1;
            } else {
                return estimate;
            }
        }
        previous_estimate = estimate;
    }
    return estimate;
}

pub fn square_root_128 (input: i128) -> i128 {
    if input <= 0 {
        /* Negative input may occur due to rounding errors when the answer should be 0.
        Therefore, it makes sense to return 0 in that case */
        return 0;
    }

    // Application of Heron's method.

    let mut power_of_100: u32 = 1;
    // Figure out the magnitude of the answer and make an estimate.
    while i128::pow(100i128, power_of_100) < input {
        power_of_100 = power_of_100 + 1;
    }

    let mut estimate;
    let mut previous_estimate = -1;

    if input / i128::pow(100i128, power_of_100 - 1) < 10 {
        estimate = 2 * i128::pow(10i128, power_of_100 - 1);
    } else {
        estimate = 6 * i128::pow(10i128, power_of_100 - 1);
    }

    // Repeat until the estimate is a better estimate that the integer above or below it.
    while
        (input - estimate * estimate).abs() > (input - (estimate + 1) * (estimate + 1)).abs() ||
        (input - estimate * estimate).abs() > (input - (estimate - 1) * (estimate - 1)).abs()
    {
        estimate = (estimate + input / estimate) / 2;
        if estimate == previous_estimate {
            // This extra code is needed because it gets stuck if the answer is closer to one larger because of integer division.
            if
                (input - estimate * estimate).abs() >
                (input - (estimate + 1) * (estimate + 1)).abs()
            {
                return estimate + 1;
            } else {
                return estimate;
            }
        }
        previous_estimate = estimate;
    }
    return estimate;
}

#[test]
fn test_square_root() {
    assert_eq!(square_root(38), 6);
    assert_eq!(square_root(100), 10);
    assert_eq!(square_root(223), 15);
}
