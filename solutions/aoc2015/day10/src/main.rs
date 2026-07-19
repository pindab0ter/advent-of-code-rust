//! [Elves Look, Elves Say](https://adventofcode.com/2015/day/10)

use aoc_client::input;
use common::timed;
use std::iter::successors;

fn main() {
    let input = input(2015, 10);
    let digits = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let mut sequence = successors(Some(digits), |x| Some(look_and_say(x)));

    println!(
        "After look-and-saying 40 times, the length of the output is {:?}",
        timed("the first 40 iterations", || sequence
            .nth(40)
            .unwrap()
            .len())
    );
    println!(
        "After look-and-saying 50 times, the length of the output is {:?}",
        timed("the next remaining sequences", || sequence
            .nth(9)
            .unwrap()
            .len())
    );
}

fn look_and_say(digits: &[u8]) -> Vec<u8> {
    let (first, rest) = digits.split_first().unwrap();
    let mut result = Vec::new();
    let mut count = 1u8;
    let mut current = *first;

    for &digit in rest {
        if digit == current {
            count += 1
        } else {
            result.push(count);
            result.push(current);
            current = digit;
            count = 1;
        }
    }

    result.push(count);
    result.push(current);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1u8], vec![1u8,1])]
    #[case(vec![1u8,1], vec![2u8,1])]
    #[case(vec![2u8,1], vec![1u8,2,1,1])]
    #[case(vec![1u8,2,1,1], vec![1u8,1,1,2,2,1])]
    #[case(vec![1u8,1,1,2,2,1], vec![3u8,1,2,2,1,1])]
    fn look_and_says(#[case] input: Vec<u8>, #[case] expected: Vec<u8>) {
        assert_eq!(look_and_say(&input), expected);
    }
}
