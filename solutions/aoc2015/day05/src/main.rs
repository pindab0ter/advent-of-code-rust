use aoc_client::input;
use common_macros::timed;

fn main() {
    let input = input(2015, 5);

    let nice_strings = count_nice_strings(&input, string_is_nice);

    println!("Found {nice_strings} nice strings");
}

#[timed]
fn count_nice_strings(input: &str) -> usize {
    input.lines().filter(|line| string_is_nice(line)).count()
}

fn contains_three_vowels(string: &str) -> bool {
    string.as_bytes().iter().filter(|c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')).take(3).count() >= 3
}

fn contains_double_letters(string: &str) -> bool {
    string.as_bytes().windows(2).any(|window| window[0] == window[1])
}

fn contains_forbidden_strings(string: &str) -> bool {
    string.as_bytes().windows(2).any(|window| matches!(window, [b'a', b'b'] | [b'c', b'd'] | [b'p', b'q'] | [b'x', b'y']))
}

fn string_is_nice(string: &str) -> bool {
    !contains_forbidden_strings(string) && contains_three_vowels(string) && contains_double_letters(string)
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
}
