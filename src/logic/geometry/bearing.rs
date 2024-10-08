use super::{Point, Vector};
use std::ops::Neg;

pub trait Bearing: PartialEq + Eq + From<Point> + Neg {
    // The vector from p to the next point along the bearing direction
    fn next_vector(p: Point) -> Option<Vector>;

    // The next n points along the bearing direction
    fn line(p: Point, n: usize) -> Vec<Point>;

    // The next point along the bearing direction
    fn next(p: Point) -> Option<Point>;
}