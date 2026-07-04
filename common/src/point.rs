use crate::vector::Vector2D;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: From<i8>> Point<T> {
    pub fn origin() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }
}

impl<T: Add<Output = T>> Add<Vector2D<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Vector2D<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign<Vector2D<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Vector2D<T>) {
        *self = *self + rhs;
    }
}
