//! [I Was Told There Would Be No Math](https://adventofcode.com/2015/day/2)

use aoc_client::input;
use common::timed;
use present::Present;
use std::str::FromStr;

mod present;

fn main() {
    let input = input(2015, 2);

    let presents: Vec<Present> = input
        .lines()
        .map(|line| Present::from_str(line).unwrap())
        .collect();

    let total_paper_area: u32 = timed("Calculating total paper area", || {
        presents
            .iter()
            .map(|present| present.required_paper_area())
            .sum()
    });
    println!("Total paper required: {total_paper_area} ft²\n");

    let total_ribbon_length: u32 = timed("Calculating required ribbon length", || {
        presents
            .iter()
            .map(|present| present.required_ribbon_length())
            .sum()
    });
    println!("Total ribbon required: {total_ribbon_length} ft");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Present{l: 2, w: 3, h: 4}, 58)]
    #[case(Present{l: 1, w: 1, h: 10}, 43)]
    fn calculates_required_paper_area(#[case] present: Present, #[case] expected: u32) {
        assert_eq!(present.required_paper_area(), expected);
    }

    #[rstest]
    #[case(Present{l: 2, w: 3, h: 4}, 34)]
    #[case(Present{l: 1, w: 1, h: 10}, 14)]
    fn calculates_required_ribbon_length(#[case] present: Present, #[case] expected: u32) {
        assert_eq!(present.required_ribbon_length(), expected);
    }
}
