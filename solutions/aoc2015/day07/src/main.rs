//! [Some Assembly Required](https://adventofcode.com/2015/day/7)

mod parse;

use crate::Source::{Binary, Direct, Not};
use aoc_client::input;
use common::timed;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;

type Wire = String;

struct Circuit {
    instructions: HashMap<Wire, Source>,
    cache: RefCell<HashMap<Wire, u16>>,
}

impl Circuit {
    fn resolve(&self, signal: &Signal) -> u16 {
        match signal {
            Signal::Value(value) => *value,
            Signal::Ref(wire) => self.get_signal(wire),
        }
    }

    fn get_signal(&self, wire: &str) -> u16 {
        if let Some(&value) = self.cache.borrow().get(wire) {
            return value;
        }
        let value = match &self.instructions[wire] {
            Direct(signal) => self.resolve(signal),
            Binary { lhs, op, rhs } => {
                let a = self.resolve(lhs);
                let b = self.resolve(rhs);

                match op {
                    BinaryOperation::And => a & b,
                    BinaryOperation::Or => a | b,
                    BinaryOperation::LeftShift => a << b,
                    BinaryOperation::RightShift => a >> b,
                }
            }
            Not(signal) => !self.resolve(signal),
        };

        self.cache.borrow_mut().insert(wire.to_string(), value);

        value
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Signal {
    Value(u16),
    Ref(Wire),
}

#[derive(Debug, Clone)]
enum BinaryOperation {
    And,
    Or,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone)]
enum Source {
    Direct(Signal),
    Not(Signal),
    Binary {
        lhs: Signal,
        op: BinaryOperation,
        rhs: Signal,
    },
}

#[derive(Debug)]
struct Instruction {
    source: Source,
    destination: Wire,
}

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?<a>[\da-z]+) )?(?:(?<operation>AND|OR|LSHIFT|RSHIFT|NOT) )?(?<b>[\da-z]+) -> (?<target>.+)$").unwrap()
});

fn parse(input: String) -> Circuit {
    Circuit {
        instructions: input
            .lines()
            .map(|l| l.into())
            .map(|instruction: Instruction| (instruction.destination, instruction.source))
            .collect(),
        cache: RefCell::from(HashMap::new()),
    }
}

fn main() {
    let input = input(2015, 7);

    let mut circuit = parse(input);

    let signal = timed("Getting signal", || circuit.get_signal("a"));
    println!("Wire ‘a’ holds value {signal}\n");

    // Update for part 2
    circuit.instructions.insert(
        "b".to_string(),
        Direct(Signal::Value(circuit.get_signal("a"))),
    );
    circuit.cache.borrow_mut().clear();

    let new_signal = timed("Getting signal", || circuit.get_signal("a"));
    println!("Wire ‘a’ now holds value {new_signal}");
}

#[cfg(test)]
mod tests {
    use crate::{Circuit, parse};
    use indoc::indoc;

    #[test]
    fn assembles_the_circuit() {
        let input = indoc! {"
            123 -> x
            456 -> y
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> i
        "}
        .into();

        let circuit: Circuit = parse(input);

        assert_eq!(circuit.get_signal("d"), 72);
        assert_eq!(circuit.get_signal("e"), 507);
        assert_eq!(circuit.get_signal("f"), 492);
        assert_eq!(circuit.get_signal("g"), 114);
        assert_eq!(circuit.get_signal("h"), 65412);
        assert_eq!(circuit.get_signal("i"), 65079);
        assert_eq!(circuit.get_signal("x"), 123);
        assert_eq!(circuit.get_signal("y"), 456);
    }

    #[test]
    fn allows_wire_identifiers_before_they_are_defined() {
        let input = indoc! {"
            x -> y
            123 -> x
        "}
        .into();

        let circuit = parse(input);

        let y = circuit.get_signal("y");

        assert_eq!(y, 123u16);
    }
}
