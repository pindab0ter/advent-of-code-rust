//! [All in a Single Night](https://adventofcode.com/2015/day/9)

use aoc_client::input;
use common::timed;
use common_macros::timed;
use itertools::Itertools;
use std::convert::Infallible;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

type Distance = u32;

/// [`distances`] is a 2D matrix of distances indexed by
/// `source_city_id + [city_count] * destination_city_id`. See [`distance_index`].
#[derive(Debug)]
struct DistanceMatrix {
    city_count: usize,
    distances: Vec<Option<Distance>>,
}

impl DistanceMatrix {
    fn new(n: usize) -> Self {
        Self {
            city_count: n,
            distances: vec![None; n * n],
        }
    }
}

impl Index<(usize, usize)> for DistanceMatrix {
    type Output = Option<Distance>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.distances[index.0 * self.city_count + index.1]
    }
}

impl IndexMut<(usize, usize)> for DistanceMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.distances[index.0 * self.city_count + index.1]
    }
}

impl FromStr for DistanceMatrix {
    type Err = Infallible;

    #[timed]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cities_to_distances = s
            .lines()
            .map(|line| {
                let (source, rest) = line.split_once(" to ").unwrap();
                let (destination, distance) = rest.split_once(" = ").unwrap();

                ((source, destination), distance.parse().unwrap())
            })
            .collect::<Vec<((&str, &str), Distance)>>();

        let mut cities = cities_to_distances
            .iter()
            .flat_map(|((source, destination), _)| [*source, *destination])
            .collect::<Vec<&str>>();
        cities.sort();
        cities.dedup();

        let mut matrix = DistanceMatrix::new(cities.len());

        for ((source, destination), distance) in cities_to_distances {
            let source_id = cities.binary_search(&source).unwrap();
            let destination_id = cities.binary_search(&destination).unwrap();
            matrix[(source_id, destination_id)] = Some(distance);
            matrix[(destination_id, source_id)] = Some(distance);
        }

        Ok(matrix)
    }
}

fn main() {
    let input = input(2015, 9);
    let distance_matrix = input.parse().unwrap();

    let (shortest_route, longest_route) = timed("calculating shortest and longest route", || {
        route_distances(&distance_matrix)
            .minmax()
            .into_option()
            .unwrap()
    });

    println!("The shortest route visiting all destinations is: {shortest_route}");
    println!("The longest route visiting all destinations is: {longest_route}");
}

fn distance_index(destination_id: usize, source_id: usize, city_count: usize) -> usize {
    source_id * city_count + destination_id
}

fn route_distances(matrix: &DistanceMatrix) -> impl Iterator<Item = Distance> {
    let all_possible_routes = (0..matrix.city_count).permutations(matrix.city_count);

    all_possible_routes.map(|route| {
        route
            .windows(2)
            .map(|hop| matrix.distances[distance_index(hop[0], hop[1], matrix.city_count)].unwrap())
            .sum::<Distance>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "};

    #[test]
    fn finds_the_shortest_route() {
        let distance_matrix = EXAMPLE.parse().unwrap();
        let (shortest_route, _) = route_distances(&distance_matrix)
            .minmax()
            .into_option()
            .unwrap();

        assert_eq!(shortest_route, 605);
    }

    #[test]
    fn finds_the_longest_route() {
        let distance_matrix = EXAMPLE.parse().unwrap();
        let (_, longest_route) = route_distances(&distance_matrix)
            .minmax()
            .into_option()
            .unwrap();

        assert_eq!(longest_route, 982);
    }
}
