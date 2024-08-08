// This utility function calculates square root for integers.

pub fn square_root(input: i64) -> i64 {
    if input < 0 {
        panic!("Square root of negative number.");
    }

    let mut n = 0;

    while n * n < input {
        n = n + 1;
    }

    if input - (n - 1) * (n - 1) < n * n - input {
        return n - 1;
    } else {
        return n;
    }
}

#[test]
fn test_square_root() {
    assert_eq!(square_root(38), 6);
    assert_eq!(square_root(100), 10);
    assert_eq!(square_root(223), 15)
}
