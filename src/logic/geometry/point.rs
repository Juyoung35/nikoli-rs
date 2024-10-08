use super::Vector;
use std::ops::{Neg, Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn center(points: &Vec<Self>) -> Self {
        // currently not handling error of length of points can be zero.
        let (x, y) = points.iter().fold((0, 0), |acc, e| (acc.x + e.x, acc.y + e.y));
        Self::new(x / points.len(), y / points.len())
    }
    // pub fn compare()
    // Whether the line segment p1-p2 intersects the line segment q1-q2
    pub fn intersect(p1: Self, p2: Self, q1: Self, q2: Self) -> bool {
        let (dp, dq) = (p2 - p1, q2 - q1);
        (q1 - p1).into().cross_product(dp) * (q2 - p1).into().cross_product(dp) < 0 &&
        (p1 - q1).into().cross_product(dq) * (p2 - q1).into().cross_product(dq) < 0
    }
}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self {
        Self::new(value.dx, value.dy)
    }
}
impl Into<Vector> for Point {
    fn into(self) -> Vector {
        Vector::new(self.x, self.y)
    }
}
impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -self.into()
    }
}
impl Add<Rhs = Self> for Point {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        self.into() + rhs.into()
    }
}
impl Add<Rhs = Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        (self.into() + rhs).into()
    }
}
impl Sub<Rhs = Self> for Point {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        self.into() - rhs.into()
    }
}
impl Sub<Rhs = Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        (self.into() - rhs).into()
    }
}