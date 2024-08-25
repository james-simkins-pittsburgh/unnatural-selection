// This module solves quadratics using the quadratic formula.

use super::integer_math::square_root;

pub fn solve_quadratic (a_x_1000: i64, b_x_1000:i64, c_x_1000:i64) -> (i64, i64) {

    return ((-b_x_1000 + square_root(b_x_1000 * b_x_1000 - 4 * a_x_1000 * c_x_1000 )) / (2 * a_x_1000 ) , (-b_x_1000 - square_root(b_x_1000 * b_x_1000 - 4 * a_x_1000 * c_x_1000 )) / (2 * a_x_1000 ))

    
}

#[test]
fn test_quadratic_solver() {

    assert_eq!(solve_quadratic(2, -2, -40), (5, -4));
    assert_eq!(solve_quadratic(5, -21030, 9305050), (3703, 502))

}