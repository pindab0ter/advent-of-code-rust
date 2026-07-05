use aoc_client::input;
use common_macros::timed;
use md5::{Digest, Md5};
use std::iter::successors;

fn main() {
    let input = input(2015, 4);

    let result = mine_advent_coin(&input);
    println!("Advent Coin mined at {result}");
}

#[timed]
fn mine_advent_coin(secret_key: &str) -> u32 {
    let mut secret_key_bytes = secret_key.as_bytes().to_vec();
    let base = secret_key_bytes.len();
    let mut hasher = Md5::new();
    let mut itoa = itoa::Buffer::new();

    for n in successors(Some(1), |&n| Some(n + 1)) {
        secret_key_bytes.truncate(base);
        secret_key_bytes.extend(itoa.format(n).as_bytes());
        Digest::update(&mut hasher, &secret_key_bytes);
        let digest = Digest::finalize_reset(&mut hasher);

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
