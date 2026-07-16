//! [The Ideal Stocking Stuffer](https://adventofcode.com/2015/day/4)

use aoc_client::input;
use common_macros::timed;
use md5::digest::Output;
use md5::{Digest, Md5};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::{available_parallelism, scope};

fn main() {
    let input = input(2015, 4);

    let first_answer = mine_advent_coin(&input, has_five_leading_hex_zeroes, 0);
    println!("Advent Coin starting with 5 zeroes mined at {first_answer}\n");

    let second_answer = mine_advent_coin(&input, has_six_leading_hex_zeroes, first_answer);
    println!("Advent Coin starting with 6 zeroes mined at {second_answer}");
}

fn has_five_leading_hex_zeroes(value: Output<Md5>) -> bool {
    value[..2] == [0, 0] && value[2] & 0xf0 == 0
}

fn has_six_leading_hex_zeroes(value: Output<Md5>) -> bool {
    value[..3] == [0, 0, 0]
}

#[timed]
fn mine_advent_coin<F: Fn(Output<Md5>) -> bool + Sync>(
    secret_key: &str,
    is_valid_advent_coin: F,
    start: u32,
) -> u32 {
    let secret_key_bytes = secret_key.as_bytes();
    let base = secret_key_bytes.len();
    let best = &AtomicU32::new(u32::MAX);
    let is_valid_advent_coin = &is_valid_advent_coin;

    scope(|scope| {
        let thread_count = available_parallelism().unwrap().into();
        for thread_index in 0_usize..thread_count {
            let mut secret_key_bytes = secret_key_bytes.to_vec();

            scope.spawn(move || {
                let mut hasher = Md5::new();
                let mut itoa = itoa::Buffer::new();

                for i in (start as usize + thread_index..).step_by(thread_count) {
                    if best.load(Relaxed) < i as u32 {
                        return;
                    }

                    secret_key_bytes.truncate(base);
                    secret_key_bytes.extend(itoa.format(i).as_bytes());
                    Digest::update(&mut hasher, &secret_key_bytes);
                    let digest = Digest::finalize_reset(&mut hasher);

                    if is_valid_advent_coin(digest) {
                        best.fetch_min(i as u32, Relaxed);
                    }
                }
            });
        }
    });
    best.load(Relaxed)
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
            mine_advent_coin(input, has_five_leading_hex_zeroes, 0),
            expected
        );
    }
}
