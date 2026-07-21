//! [Knights of the Dinner Table](https://adventofcode.com/2015/day/13)

use aoc_client::input;
use common::grid::Grid;
use common_macros::timed;
use itertools::Itertools;

type GuestId = usize;
type Happiness = i64;

fn main() {
    let input = input(2015, 13);

    let preferences = parse(&input);
    let guest_count = preferences.width;
    let seating_arrangements = generate_seating_arrangements(guest_count);
    let happiness_per_arrangement = calculate_happiness(seating_arrangements, preferences);

    println!(
        "The total change in happiness for the optimal arrangement is {}",
        happiness_per_arrangement.iter().max().unwrap()
    );
}

#[timed]
fn calculate_happiness(
    seating_arrangements: Vec<Vec<GuestId>>,
    preferences: Grid<Happiness>,
) -> Vec<Happiness> {
    let guest_count = preferences.width;

    seating_arrangements
        .into_iter()
        .map(|seating_arrangement| {
            (0..guest_count)
                .map(|i| {
                    let one = seating_arrangement[i];
                    let other = seating_arrangement[(i + 1) % guest_count];
                    preferences[(one, other)] + preferences[(other, one)]
                })
                .sum()
        })
        .collect::<Vec<Happiness>>()
}

fn generate_seating_arrangements(guest_count: usize) -> Vec<Vec<GuestId>> {
    (1..guest_count)
        .permutations(guest_count - 1)
        .map(|mut permutation| {
            permutation.push(0);
            permutation
        })
        .collect()
}

#[timed]
fn parse(input: &str) -> Grid<Happiness> {
    let guest_count = input.lines().count().isqrt() + 1;

    let names = input
        .lines()
        .map(|line| {
            let (name, _) = line.split_once(" would ").unwrap();
            name.into()
        })
        .unique()
        .sorted()
        .collect::<Vec<String>>();

    input
        .lines()
        .fold(Grid::new(guest_count, guest_count), |mut grid, line| {
            let (guest_name, rest) = line.split_once(" would ").unwrap();
            let guest_id = names.iter().position(|n| n == guest_name).unwrap();

            let (modifier, rest) = rest.split_once(" ").unwrap();
            let (amount, rest) = rest
                .split_once(" happiness units by sitting next to ")
                .unwrap();
            let amount =
                amount.parse::<Happiness>().unwrap() * if modifier == "lose" { -1 } else { 1 };

            let (neighbor_name, _) = rest.split_once(".").unwrap();
            let neighbor_id = names.iter().position(|n| n == neighbor_name).unwrap();

            grid[(guest_id, neighbor_id)] = amount;

            grid
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "};

    #[test]
    fn finds_the_highest_happiness() {
        let preferences = parse(EXAMPLE);
        let guest_count = preferences.width;
        let seating_arrangements = generate_seating_arrangements(guest_count);
        let happiness_per_arrangement = calculate_happiness(seating_arrangements, preferences);

        assert_eq!(*happiness_per_arrangement.iter().max().unwrap(), 330);
    }
}
