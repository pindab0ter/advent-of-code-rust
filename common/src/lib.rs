pub mod direction;
pub mod grid;
pub mod point;
pub mod vector;

pub use direction::Direction;

use std::time::Instant;

pub fn timed<T>(name: &str, work: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = work();

    println!("{name} took {:.0?}", start.elapsed());

    result
}
