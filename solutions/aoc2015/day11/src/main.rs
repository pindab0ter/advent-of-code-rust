//! [Corporate Policy](https://adventofcode.com/2015/day/11)

use crate::password::Password;
use aoc_client::input;

mod password;

fn main() {
    let input = input(2015, 11);
    let mut password = input.parse::<Password>().unwrap();

    password.next_valid_password();
    println!("The next valid password is: {password:?}\n");

    password.next_valid_password();
    println!("The next valid password after that is: {password:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // Provided cases
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdefgh", false)]
    // Custom cases
    #[case("aaadefgh", false)]
    fn validates_password(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(input.parse::<Password>().unwrap().is_valid(), expected);
    }

    #[rstest]
    // Provided cases
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    // Custom cases
    #[case("aaaaaaaz", "aaaaaaba")]
    fn finds_next_valid_password(#[case] input: &str, #[case] expected: &str) {
        let mut password = input.parse::<Password>().unwrap();
        password.next_valid_password();
        assert_eq!(password, expected);
    }
}
