use crate::Action::{Toggle, TurnOff, TurnOn};
use aoc_client::input;
use common::grid::Grid;
use common::point::Point;
use common_macros::timed;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    point_a: Point<usize>,
    point_b: Point<usize>,
}

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?<action>turn on|turn off|toggle) (?<ax>\d+),(?<ay>\d+) through (?<bx>\d+),(?<by>\d+)",
    )
    .unwrap()
});

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE.captures(s).unwrap();

        Ok(Instruction {
            action: Action::from_str(&caps["action"])?,
            point_a: Point {
                x: usize::from_str(&caps["ax"]).unwrap(),
                y: usize::from_str(&caps["ay"]).unwrap(),
            },
            point_b: Point {
                x: usize::from_str(&caps["bx"]).unwrap(),
                y: usize::from_str(&caps["by"]).unwrap(),
            },
        })
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(TurnOn),
            "turn off" => Ok(TurnOff),
            "toggle" => Ok(Toggle),
            _ => Err(()),
        }
    }
}

#[timed]
fn count_lit_lights(instructions: &[Instruction]) -> usize {
    let mut grid = Grid::<bool>::new(1000, 1000);
    for instruction in instructions {
        match instruction.action {
            TurnOn => grid.set_rect(instruction.point_a, instruction.point_b, true),
            TurnOff => grid.set_rect(instruction.point_a, instruction.point_b, false),
            Toggle => grid.map_rect(instruction.point_a, instruction.point_b, |cell| !cell),
        }
    }

    grid.iter().filter(|cell| **cell).count()
}

fn main() {
    let input = input(2015, 6);

    let instructions = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>();

    let lit_lights_count = count_lit_lights(&instructions);
    println!("{lit_lights_count} lights are lit after following the instructions");
}

#[cfg(test)]
mod tests {
    use crate::{count_lit_lights, Instruction};
    use rstest::rstest;
    use std::str::FromStr;

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1_000_000)]
    #[case("turn on 0,0 through 999,0", 1_000)]
    #[case("turn on 499,499 through 500,500", 4)]
    fn counts_lit_lights(#[case] input: &str, #[case] expected: usize) {
        let instruction = Instruction::from_str(input).unwrap();
        let count = count_lit_lights(&[instruction]);
        assert_eq!(count, expected);
    }
}
