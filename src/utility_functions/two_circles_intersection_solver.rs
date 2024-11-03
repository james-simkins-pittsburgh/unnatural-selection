/* This find the intersection points of two circles. The code assumes the circles do intersect 
and does not consider non-intersecting circles. Special thanks to Tim Voght who wrote the code in C 
that this code was adapted from and generously placed it in the public domain. */

use super::integer_math::square_root_64;

pub fn solve_two_circle_intersection(
    x0: i32,
    y0: i32,
    r0: i32,
    x1: i32,
    y1: i32,
    r1: i32
) -> ((i32, i32), (i32, i32)) {

    let dx = x1 - x0;

    let dy = y1 - y0;

    let d = square_root_64((dx as i64) * (dx as i64) + (dy as i64) * (dy as i64)) as i32;

    let a: i32;

    if d != 0 {
        a = (r0 * r0 - (r1 * r1) + d * d) / (2 * d);
    } else {
        return ((0, 0), (0, 0));
    }

    let x2 = x0 + (dx * a/d);

    let y2 = y0 + (dy * a/d);

    let h = square_root_64((r0 * r0 - a * a) as i64) as i32;

    let rx = -dy * h / d;

    let ry = dx * h / d;

    return ((x2 + rx, y2 + ry),(x2 - rx, y2 - ry))
}

#[test] 

fn test_circle_intersection_solver () {
    assert_eq!(solve_two_circle_intersection (3230, 4213, 2000, 7324, 3413, 3000), ((4944,5242),(4432,2614)));
    assert_eq!(solve_two_circle_intersection (-200, -250, 630, 150, -400, 447), ((386,-20),(38,-832)));
}