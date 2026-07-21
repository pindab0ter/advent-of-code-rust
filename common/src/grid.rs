use crate::point::Point;
use std::cmp::{max, min};
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::iter::repeat_with;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn offset(&self, index: impl GridIndex) -> usize {
        let (x, y) = index.coords();
        assert!(
            x < self.width && y < self.height,
            "point ({x},{y}) out of bounds ({},{})",
            self.width,
            self.height
        );

        x + y * self.width
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
}

// region Constructors
impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            data: repeat_with(T::default).take(width * height).collect(),
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn from_element(width: usize, height: usize, value: T) -> Self {
        Grid {
            width,
            height,
            data: vec![value; width * height],
        }
    }
}
// endregion

// region Rectangle-in-grid
impl<T: Copy> Grid<T> {
    fn rect_bounds(&self, a: impl GridIndex, b: impl GridIndex) -> (usize, usize, usize, usize) {
        let (ax, ay) = a.coords();
        let (bx, by) = b.coords();

        assert!(
            ax < self.width && ay < self.height,
            "point a ({ax},{ay}) out of bounds ({},{})",
            self.width,
            self.height
        );

        assert!(
            bx < self.width && by < self.height,
            "point b ({bx},{by}) out of bounds ({},{})",
            self.width,
            self.height
        );

        (min(ax, bx), max(ax, bx), min(ay, by), max(ay, by))
    }

    pub fn set_rect(&mut self, a: impl GridIndex, b: impl GridIndex, value: T) {
        let (x_min, x_max, y_min, y_max) = self.rect_bounds(a, b);

        for y in y_min..=y_max {
            let start = y * self.width + x_min;
            let end = y * self.width + x_max;
            self.data[start..=end].fill(value)
        }
    }

    pub fn map_rect<F: Fn((usize, usize), T) -> T>(
        &mut self,
        corner_a: impl GridIndex,
        corner_b: impl GridIndex,
        callback: F,
    ) {
        let (x_min, x_max, y_min, y_max) = self.rect_bounds(corner_a, corner_b);

        for y in y_min..=y_max {
            let start = y * self.width + x_min;
            let end = y * self.width + x_max;
            for (delta_x, cell) in self.data[start..=end].iter_mut().enumerate() {
                *cell = callback((x_min + delta_x, y), *cell);
            }
        }
    }
}
// endregion

// region Indexing
pub trait GridIndex {
    fn coords(self) -> (usize, usize);
}

impl GridIndex for (usize, usize) {
    fn coords(self) -> (usize, usize) {
        self
    }
}

impl GridIndex for Point<usize> {
    fn coords(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl<T, I: GridIndex> Index<I> for Grid<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[self.offset(index)]
    }
}

impl<T, I: GridIndex> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let offset = self.offset(index);
        &mut self.data[offset]
    }
}
// endregion

// region Display
struct CountLen(usize);

impl Write for CountLen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0 += s.len();
        Ok(())
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_width = self.data.iter().fold(0, |acc: usize, cell| {
            let mut count_len = CountLen(0);
            write!(count_len, "{cell}").unwrap();
            max(acc, count_len.0)
        });

        for y in 0..self.height {
            for x in 0..self.width {
                if x > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{:>w$}", self.data[x + y * self.width], w = max_width,)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
// endregion
