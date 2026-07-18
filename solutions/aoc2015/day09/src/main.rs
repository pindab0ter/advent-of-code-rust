//! [All in a Single Night](https://adventofcode.com/2015/day/9)

use aoc_client::input;
use common_macros::timed;
use std::collections::{HashMap, HashSet};
use crate::collections::permutations;

mod collections;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Hop<'a>(&'a str, &'a str);

impl<'a> Hop<'a> {
    fn new(a: &'a str, b: &'a str) -> Self {
        if a <= b { Hop(a, b) } else { Hop(b, a) }
    }
}

fn main() {
    let input = input(2015, 9);

    let hop_distances = parse(&input);
    let all_possible_routes = all_possible_routes(&hop_distances);
    let shortest_route = shortest_route(&hop_distances, &all_possible_routes);

    println!("The shortest route visiting all destinations is: {shortest_route}");
}

#[timed]
fn parse(input: &str) -> HashMap<Hop<'_>, u32> {
    input.lines().fold(HashMap::new(), |mut distances, line| {
        let (source, rest) = line.split_once(" to ").unwrap();
        let (destination, distance) = rest.split_once(" = ").unwrap();

        distances.insert(Hop::new(source, destination), distance.parse().unwrap());

        distances
    })
}

#[timed]
fn all_possible_routes<'a>(distances: &HashMap<Hop<'a>, u32>) -> Vec<Vec<&'a str>> {
    permutations(
        &distances
            .keys()
            .flat_map(|hop| [hop.0, hop.1])
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect::<Vec<&str>>(),
    )
    .collect::<Vec<Vec<&str>>>()
}

#[timed]
fn shortest_route(distances: &HashMap<Hop, u32>, all_possible_routes: &Vec<Vec<&str>>) -> u32 {
    all_possible_routes
        .iter()
        .map(|destinations| {
            destinations
                .windows(2)
                .map(|hop| distances[&Hop::new(hop[0], hop[1])])
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1() {
        let input = "
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
        "
        .trim();

        let distances = parse(input);
        let all_possible_routes = all_possible_routes(&distances);
        let shortest_route = shortest_route(&distances, &all_possible_routes);

        assert_eq!(shortest_route, 605);
    }
}
