use aoc_client::input;
use common_macros::timed;

fn main() {
    let input = input(2015, 5);

    let nice_strings = count_nice_strings(&input, string_is_nice);
    println!("Found {nice_strings} nice strings\n");

    let nicer_strings = count_nice_strings(&input, string_is_nicer);
    println!("Found {nicer_strings} nicer strings");
}

#[timed]
fn count_nice_strings<F: Fn(&str) -> bool>(input: &str, nice_check: F) -> usize {
    input.lines().filter(|line| nice_check(line)).count()
}

// Part 1

fn contains_three_vowels(string: &str) -> bool {
    string
        .as_bytes()
        .iter()
        .filter(|c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u'))
        .nth(3)
        .is_some()
}

fn contains_double_letters(string: &str) -> bool {
    string
        .as_bytes()
        .windows(2)
        .any(|window| window[0] == window[1])
}

fn contains_forbidden_strings(string: &str) -> bool {
    string.as_bytes().windows(2).any(|window| {
        matches!(
            window,
            [b'a', b'b'] | [b'c', b'd'] | [b'p', b'q'] | [b'x', b'y']
        )
    })
}

fn string_is_nice(string: &str) -> bool {
    !contains_forbidden_strings(string)
        && contains_three_vowels(string)
        && contains_double_letters(string)
}

// Part 2

fn contains_non_overlapping_pairs(string: &str) -> bool {
    let last_pair_start = string.len().saturating_sub(1);
    let pair_indices = 0..last_pair_start;

    // Walk through the string and see if a pair exists in the rest of the string
    pair_indices.to_owned().any(|i| {
        let pair = &string[i..=i + 1];
        let rest = &string[i + 2..];
        rest.contains(pair)
    })
}

fn contains_split_duo(string: &str) -> bool {
    string
        .as_bytes()
        .windows(3)
        .any(|window| window.first() == window.last())
}

fn string_is_nicer(string: &str) -> bool {
    contains_non_overlapping_pairs(string) && contains_split_duo(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn determines_nice_strings(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(string_is_nice(input), expected);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn determines_nicer_strings(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(string_is_nicer(input), expected);
    }
}
