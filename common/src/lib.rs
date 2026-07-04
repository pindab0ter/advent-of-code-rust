pub mod direction;
pub mod vector;
pub mod point;

pub use direction::Direction;

use std::time::Instant;

pub fn timed<T>(name: &str, work: impl FnOnce() -> T) -> T {
    let start = Instant::now();

    let result = work();

    println!("{name} took {} µs", start.elapsed().as_micros());

    result
}
