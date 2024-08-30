// This find the intersection points of two circles.

// The code assumes the circles do intersect and does not consider non-intersecting circles.

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
        a = (r0 * r0 - (r1 + r1) + d * d) / 2d;
    } else {
        return (0, 0);
    }

    let h = square_root_64((r0 * r0 - a * a) as i64) as i32;

    let rx = -dy * (h / d);

    let ry = dx * (h / d);
}
