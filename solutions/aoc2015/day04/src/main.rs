use aoc_client::input;
use common_macros::timed;
use std::fmt::Write;
use std::iter::successors;

fn main() {
    let input = input(2015, 4);

    let result = mine_advent_coin(&input);
    println!(
        "The lowest number it takes to make an MD5 hash starting with five zeroes is {result}"
    );
}

#[timed]
fn mine_advent_coin(secret_key: &str) -> u32 {
    let mut buf = String::new();

    for n in successors(Some(1), |&n| Some(n + 1)) {
        buf.clear();
        write!(buf, "{secret_key}{n}").unwrap();
        let digest = md5::compute(&buf);

        // Each byte holds two hex digits
        if digest[..2] == [0, 0] && digest[2] & 0xf0 == 0 {
            return n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn mines_advent_coins(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(mine_advent_coin(input), expected);
    }
}
