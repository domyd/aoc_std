use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::vector::Vector2;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Point2 {
    pub x: isize,
    pub y: isize,
}

impl Point2 {
    pub fn new() -> Self {
        Point2 { x: 0, y: 0 }
    }

    pub fn min(&self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Add<Vector2> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2> for Point2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point2> for Point2 {
    type Output = Vector2;

    fn sub(self, rhs: Point2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Vector2> for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2> for Point2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Point2 {
    type Output = Point2;

    fn neg(self) -> Self::Output {
        Point2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<isize> for Point2 {
    type Output = Point2;

    fn mul(self, rhs: isize) -> Self::Output {
        Point2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for Point2 {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<isize> for Point2 {
    type Output = Point2;

    fn div(self, rhs: isize) -> Self::Output {
        Point2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<isize> for Point2 {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
