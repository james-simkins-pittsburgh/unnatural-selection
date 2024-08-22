// This module solves quadratics using the quadratic formula.

use super::integer_math::square_root;

pub fn solve_quadratic (a: i64, b:i64, c:i64) -> (i64, i64) {

    return ((-b + square_root(b * b - 4 * a * c )) / (a * a), (-b - square_root(b * b - 4 * a * c )) / (a * a))

}