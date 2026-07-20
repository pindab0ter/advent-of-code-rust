//! [JSAbacusFramework.io](https://adventofcode.com/2015/day/12)

use crate::Mode::{All, SkipRed};
use aoc_client::input;
use common::timed;
use json::JsonValue;
use json::JsonValue::*;

fn main() {
    let input = input(2015, 12);
    let json = json::parse(&input).unwrap();

    let sum_of_all_numbers = timed("summing all numbers", || sum_all_numbers(&json, All));
    println!("The sum of all numbers in the document is {sum_of_all_numbers}\n");

    let sum_of_all_non_red_numbers = timed("summing all non-red numbers", || {
        sum_all_numbers(&json, SkipRed)
    });
    println!("The sum of all non-red numbers in the document is {sum_of_all_non_red_numbers}");
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    All,
    SkipRed,
}

fn sum_all_numbers(json_value: &JsonValue, mode: Mode) -> i64 {
    match json_value {
        Number(number) => number.as_fixed_point_i64(0).unwrap(),
        Object(object) => {
            if mode == SkipRed && object.iter().any(|(_, v)| v.as_str() == Some("red")) {
                return 0;
            }

            object
                .iter()
                .map(|(_, value)| sum_all_numbers(value, mode))
                .sum::<i64>()
        }
        Array(array) => array
            .iter()
            .map(|value| sum_all_numbers(value, mode))
            .sum::<i64>(),

        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case(r#"[-1,{"a":1}]"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn sums_all_numbers(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(sum_all_numbers(&json::parse(input).unwrap(), All), expected);
    }

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn sums_all_non_red_numbers(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(
            sum_all_numbers(&json::parse(input).unwrap(), SkipRed),
            expected
        );
    }
}
