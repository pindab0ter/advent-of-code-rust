//! [All in a Single Night](https://adventofcode.com/2015/day/9)

use aoc_client::input;
use common_macros::timed;
use itertools::Itertools;

type Distance = u32;

/// [`distances`] is a 2D matrix of distances indexed by
/// `source_city_id * [city_count] * destination_city_id`. See [`distance_index`].
#[derive(Debug)]
struct DistanceMatrix {
    city_count: usize,
    distances: Vec<Option<Distance>>,
}

impl DistanceMatrix {
    fn get_distance_for(&self, source: usize, destination: usize) -> Distance {
        self.distances[distance_index(source, destination, self.city_count)].unwrap()
    }
}

fn main() {
    let input = input(2015, 9);

    let distance_matrix = parse(&input);
    let shortest_route = find_shortest_route(distance_matrix);

    println!("The shortest route visiting all destinations is: {shortest_route}");
}

fn distance_index(destination_id: usize, source_id: usize, cities_count: usize) -> usize {
    source_id * cities_count + destination_id
}

#[timed]
fn parse(input: &str) -> DistanceMatrix {
    let cities_to_distances = input
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

    let city_count = cities.len();

    let mut distances = vec![None; city_count * city_count];

    for ((source, destination), distance) in cities_to_distances {
        let source_id = cities.binary_search(&source).unwrap();
        let destination_id = cities.binary_search(&destination).unwrap();
        distances[distance_index(destination_id, source_id, city_count)] = Some(distance);
        distances[distance_index(source_id, destination_id, city_count)] = Some(distance);
    }

    DistanceMatrix {
        city_count,
        distances,
    }
}

#[timed]
fn find_shortest_route(matrix: DistanceMatrix) -> u32 {
    let all_possible_routes = (0..matrix.city_count).permutations(matrix.city_count);

    all_possible_routes
        .map(|route| {
            route
                .windows(2)
                .map(|hop| matrix.get_distance_for(hop[0], hop[1]))
                .sum::<u32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_shortest_distance() {
        let input = "
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
        "
        .trim();

        let distance_matrix = parse(input);
        let shortest_route = find_shortest_route(distance_matrix);

        assert_eq!(shortest_route, 605);
    }
}
