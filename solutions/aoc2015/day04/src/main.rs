use aoc_client::input;
use common_macros::timed;
use md5::digest::Output;
use md5::{Digest, Md5};
use std::iter::successors;

fn main() {
    let input = input(2015, 4);

    let first_five_leading_zeroes = mine_advent_coin(&input, has_five_leading_hex_zeroes);
    println!("Advent Coin starting with 5 zeroes mined at {first_five_leading_zeroes}\n");

    let first_six_leading_zeroes = mine_advent_coin(&input, has_six_leading_hex_zeroes);
    println!("Advent Coin starting with 6 zeroes mined at {first_six_leading_zeroes}");
}

fn has_five_leading_hex_zeroes(value: Output<Md5>) -> bool {
    value[..2] == [0, 0] && value[2] & 0xf0 == 0
}

fn has_six_leading_hex_zeroes(value: Output<Md5>) -> bool {
    value[..3] == [0, 0, 0]
}

// TODO: Improve performance with paralellization
#[timed]
fn mine_advent_coin<F: Fn(Output<Md5>) -> bool>(secret_key: &str, is_valid_advent_coin: F) -> u32 {
    let mut secret_key_bytes = secret_key.as_bytes().to_vec();
    let base = secret_key_bytes.len();
    let mut hasher = Md5::new();
    let mut itoa = itoa::Buffer::new();

    for n in successors(Some(1), |&n| Some(n + 1)) {
        secret_key_bytes.truncate(base);
        secret_key_bytes.extend(itoa.format(n).as_bytes());
        Digest::update(&mut hasher, &secret_key_bytes);
        let digest = Digest::finalize_reset(&mut hasher);

        if is_valid_advent_coin(digest) {
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
        assert_eq!(
            mine_advent_coin(input, has_five_leading_hex_zeroes),
            expected
        );
    }
}
