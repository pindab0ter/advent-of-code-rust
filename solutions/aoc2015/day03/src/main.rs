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

fn main() {
    let directions = parse(&input(2015, 3));

    let houses_visited = count_visited_houses(&directions);

    println!("Houses that receive at least one present: {houses_visited}");
}

#[timed]
fn count_visited_houses(directions: &[Direction]) -> u32 {
    let houses_except_origin =
        directions
            .iter()
            .scan(Point::<i32>::origin(), |position, direction| {
                *position += direction.delta();
                Some(*position)
            });

    once(Point::origin())
        .chain(houses_except_origin)
        .collect::<HashSet<_>>()
        .len()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn counts_visited_houses(#[case] input: &str, #[case] expected: u32) {
        let actual = count_visited_houses(&parse(input));
        assert_eq!(actual, expected);
    }
}
