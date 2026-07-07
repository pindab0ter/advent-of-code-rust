use crate::BinaryOperation::{And, LeftShift, Or, RightShift};
use crate::Signal::{Ref, Value};
use crate::Source::{Binary, Direct, Not};
use crate::{BinaryOperation, Instruction, RE, Signal, Source};

impl From<&str> for Signal {
    fn from(value: &str) -> Self {
        value.parse().map_or_else(|_| Ref(value.into()), Value)
    }
}

impl From<&str> for BinaryOperation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => And,
            "OR" => Or,
            "LSHIFT" => LeftShift,
            "RSHIFT" => RightShift,
            _ => panic!("unknown operation: {value}"),
        }
    }
}

impl Source {
    pub(crate) fn from_parts(a: Option<&str>, operation: Option<&str>, b: &str) -> Self {
        match (a, operation) {
            (None, None) => Direct(b.into()),
            (Some(av), Some(op)) => Binary {
                lhs: av.into(),
                op: op.into(),
                rhs: b.into(),
            },
            (None, Some("NOT")) => Not(b.into()),
            _ => panic!("Invalid source shape: a: {a:?}, operation: {operation:?}, b: {b}"),
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let caps = RE
            .captures(value)
            .unwrap_or_else(|| panic!("Failed unwrapping {value}"));

        Instruction {
            source: Source::from_parts(
                caps.name("a").map(|m| m.as_str()),
                caps.name("operation").map(|m| m.as_str()),
                caps["b"].into(),
            ),
            destination: caps["target"].into(),
        }
    }
}
