//! [Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3)

use aoc_client::input;
use common::Direction;
use common::Direction::{East, North, South, West};
use common::point::Point;
use common_macros::timed;
use std::collections::HashSet;
use std::iter::once;

fn parse(input: &str) -> Vec<Direction> {
    input
        .bytes()
        .map(|b| match b {
            b'^' => North,
            b'>' => East,
            b'v' => South,
            b'<' => West,
            _ => panic!("Invalid direction"),
        })
        .collect()
}

trait VisitedHouses {
    fn visited_houses(self) -> HashSet<Point<i32>>;
}

impl<'a, I: Iterator<Item = &'a Direction>> VisitedHouses for I {
    fn visited_houses(self) -> HashSet<Point<i32>> {
        let houses_except_origin = self.scan(Point::<i32>::origin(), |position, direction| {
            *position += direction.delta();
            Some(*position)
        });

        once(Point::origin())
            .chain(houses_except_origin)
            .collect::<HashSet<_>>()
    }
}

#[timed]
fn count_houses_visited_by_santa(directions: &[Direction]) -> usize {
    directions.iter().visited_houses().len()
}

#[timed]
fn count_houses_visited_by_santa_and_robot(directions: &[Direction]) -> usize {
    let houses_visited_by_santa = directions.iter().step_by(2).visited_houses();
    let houses_visited_by_robot = directions.iter().skip(1).step_by(2).visited_houses();

    houses_visited_by_santa
        .union(&houses_visited_by_robot)
        .count()
}

fn main() {
    let directions = parse(&input(2015, 3));

    let houses_visited_by_santa = count_houses_visited_by_santa(&directions);
    println!("Houses that received at least one present from Santa: {houses_visited_by_santa}\n");

    let houses_visited_by_santa_and_robot = count_houses_visited_by_santa_and_robot(&directions);
    println!(
        "Houses that receive at least one present from Santa or Robot Santa: {houses_visited_by_santa_and_robot}\n"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn counts_visited_houses(#[case] input: &str, #[case] expected: usize) {
        let actual = count_houses_visited_by_santa(&parse(input));
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("^v^v^v^v^v", 11)]
    fn counts_visited_houses_with_robot_santa(#[case] input: &str, #[case] expected: usize) {
        let actual = count_houses_visited_by_santa_and_robot(&parse(input));
        assert_eq!(actual, expected);
    }
}
