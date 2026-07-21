//! [All in a Single Night](https://adventofcode.com/2015/day/9)

use aoc_client::input;
use common::timed;
use common_macros::timed;
use itertools::Itertools;
use common::grid::Grid;

type Distance = u32;

#[timed]
fn parse(s: &str) -> Grid<Distance> {
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

    let mut grid = Grid::new(cities.len(), cities.len());

    for ((source, destination), distance) in cities_to_distances {
        let source_id = cities.binary_search(&source).unwrap();
        let destination_id = cities.binary_search(&destination).unwrap();
        grid[(source_id, destination_id)] = distance;
        grid[(destination_id, source_id)] = distance;
    }

    grid
}

fn main() {
    let input = input(2015, 9);
    let grid = parse(&input);

    let (shortest_route, longest_route) = timed("calculating shortest and longest route", || {
        route_distances(&grid)
            .minmax()
            .into_option()
            .unwrap()
    });

    println!("The shortest route visiting all destinations is: {shortest_route}");
    println!("The longest route visiting all destinations is: {longest_route}");
}

fn route_distances(grid: &Grid<Distance>) -> impl Iterator<Item = Distance> {
    let all_possible_routes = (0..grid.width).permutations(grid.width);

    all_possible_routes.map(|route| {
        route
            .windows(2)
            .map(|hop| grid[(hop[0], hop[1])])
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
        let grid = parse(EXAMPLE);
        let (shortest_route, _) = route_distances(&grid).minmax().into_option().unwrap();

        assert_eq!(shortest_route, 605);
    }

    #[test]
    fn finds_the_longest_route() {
        let grid = parse(EXAMPLE);
        let (_, longest_route) = route_distances(&grid).minmax().into_option().unwrap();

        assert_eq!(longest_route, 982);
    }
}
