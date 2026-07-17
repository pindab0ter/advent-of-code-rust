//! [Matchsticks](https://adventofcode.com/2015/day/8)

use aoc_client::input;
use common::timed;

fn main() {
    let input = input(2015, 8);
    let input_length: usize = input.lines().map(|line| line.len()).sum();

    let unescaped_lines_length: usize = timed("counting unescaped characters", || {
        input.lines().map(count_unescaped_chars).sum()
    });
    println!(
        "The number of characters of code for string literals minus the number of characters in memory for the values of the strings is {}\n",
        input_length - unescaped_lines_length
    );

    let escaped_lines_length: usize = timed("counting escaped characters", || {
        input.lines().map(count_escaped_chars).sum()
    });
    println!(
        "The number of characters of code for string literals minus the number of characters in memory for the values of the strings is {}",
        escaped_lines_length - input_length
    );
}

fn count_unescaped_chars(input: &str) -> usize {
    let inner = &input[1..input.len() - 1];
    let mut chars = inner.chars();
    let mut count = 0;

    while let Some(char) = chars.next() {
        if char == '\\'
            && let Some('x') = chars.next()
        {
            chars.next();
            chars.next();
        }

        count += 1;
    }

    count
}

fn count_escaped_chars(input: &str) -> usize {
    input.chars().fold(0, |acc, char| {
        acc + match char {
            '"' | '\\' => 2, // Count as escaped
            _ => 1,
        }
    }) + 2 // Add the surrounding quotes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, 0)]
    #[case(r#""abc""#, 3)]
    #[case(r#""aaa\"aaa""#, 7)]
    #[case(r#""\x27""#, 1)]
    fn counts_unescaped_chars(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(count_unescaped_chars(input), expected);
    }

    #[rstest]
    #[case(r#""""#, 6)]
    #[case(r#""abc""#, 9)]
    #[case(r#""aaa\"aaa""#, 16)]
    #[case(r#""\x27""#, 11)]
    fn counts_escaped_chars(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(count_escaped_chars(input), expected);
    }
}
