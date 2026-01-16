use crate::util::geometry::Coordinate;

type Vec2 = (Coordinate, Coordinate);

#[inline]
pub(crate) fn rotate_vec2(v: Vec2, angle: i32) -> Vec2 {
    let (x, y) = v;
    let (x, y) = (x as f32, y as f32);
    let (a_cos, a_sin) = ((angle as f32).cos(), (angle as f32).sin());

    ((x*a_cos - y*a_sin).round() as Coordinate,
        (x*a_sin + y*a_cos).round() as Coordinate)
} 
