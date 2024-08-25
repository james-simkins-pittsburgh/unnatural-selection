// This module solves quadratics using the quadratic formula.

use super::integer_math::square_root;

pub fn solve_quadratic (a: i64, b:i64, c:i64) -> (i64, i64) {

    return ((-b + square_root(b * b - 4 * a * c )) / (2 * a), (-b - square_root(b * b - 4 * a * c )) / (2 * a))

}

#[test]
fn test_quadratic_solver() {

    assert_eq!(solve_quadratic(2, -2, -40), (5, -4));
    assert_eq!(solve_quadratic(5, -21030, 9305050), (3703, 502))

}