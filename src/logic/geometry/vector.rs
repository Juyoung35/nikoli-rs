pub const EAST:         Vector = Vector::new( 0,  1);
pub const NORTH_EAST:   Vector = Vector::new( 1,  1);
pub const NORTH:        Vector = Vector::new( 1,  0);
pub const NORTH_WEST:   Vector = Vector::new(-1,  1);
pub const WEST:         Vector = Vector::new(-1,  0);
pub const SOUTH_WEST:   Vector = Vector::new(-1, -1);
pub const SOUTH:        Vector = Vector::new( 0, -1);
pub const SOUTH_EAST:   Vector = Vector::new( 1, -1);

use std::ops::{Add, Sub, Neg, Mul};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vector {
    pub dx: isize,
    pub dy: isize,
}
impl Vector {
    pub const fn new(dx: isize, dy: isize) -> Self {
        Self { dx ,dy }
    }
    pub const fn angle(self) -> f64 {
        f64::atan2(self.dy as f64, self.dx as f64)
    }
    pub const fn cross_product(self, other: Self) -> isize {
        self.dx * other.dy - self.dy * other.dx
    }
    pub const fn dot_product(self, other: Self) -> isize {
        self.dx * other.dx + self.dy * other.dy
    }
}
impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.dx, -self.dy)
    }
}
impl Add<Rhs = Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        Self::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}
impl Sub(Rhs = Self) for Vector {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        self + (-rhs)
    }
}
impl Mul<Rhs = isize> for Vector {
    type Output = Self;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Self::new(self.dx * rhs, self.dy * rhs)
    }
}