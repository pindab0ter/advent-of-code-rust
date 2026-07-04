use crate::vector::Vector2D;
use Direction::{East, North, South, West};
use std::fmt::{Debug, Formatter};

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn delta<T: From<i8>>(&self) -> Vector2D<T> {
        match self {
            North => Vector2D {
                x: 0.into(),
                y: (-1).into(),
            },
            East => Vector2D {
                x: 1.into(),
                y: 0.into(),
            },
            South => Vector2D {
                x: 0.into(),
                y: 1.into(),
            },
            West => Vector2D {
                x: (-1).into(),
                y: 0.into(),
            },
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            North => write!(f, "⮝"),
            East => write!(f, "⮞"),
            South => write!(f, "⮟"),
            West => write!(f, "⮜"),
        }
    }
}
