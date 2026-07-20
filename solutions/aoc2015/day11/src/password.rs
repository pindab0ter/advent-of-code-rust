use common_macros::timed;
use itertools::Itertools;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub(crate) struct Password([u8; 8]);

impl Password {
    pub(crate) fn is_valid(&self) -> bool {
        if self.0.iter().any(|c| matches!(c, b'i' | b'o' | b'l')) {
            return false;
        }

        // Contains three consecutive increasing letters
        if !self
            .0
            .windows(3)
            .any(|window| window[1] == window[0] + 1 && window[2] == window[0] + 2)
        {
            return false;
        }

        // Contains at least two unique pairs of letters
        if self
            .0
            .windows(2)
            .filter(|pair| pair[0] == pair[1])
            .unique()
            .count()
            < 2
        {
            return false;
        }

        true
    }

    #[timed]
    pub(crate) fn next_valid_password(&mut self) {
        self.iterate_password();
        while !self.is_valid() {
            self.iterate_password();
        }
    }

    fn iterate_password(&mut self) {
        for d in self.0.iter_mut().rev() {
            if *d == b'z' {
                *d = b'a'
            } else {
                *d += 1;
                break;
            }
        }

        if let Some(i) = self.0.iter().position(|&d| matches!(d, b'i' | b'o' | b'l')) {
            self.0[i] += 1;
            self.0[i + 1..].fill(b'a');
        }
    }
}

impl FromStr for Password {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Password(s.as_bytes().try_into().unwrap()))
    }
}

impl PartialEq<&str> for Password {
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(&self.0))
    }
}
