use std::num::ParseIntError;
use aoc_client::input;
use std::str::FromStr;

#[derive(Clone, Copy)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn smallest_area(self) -> u32 {
        *self.faces().iter().min().unwrap()
    }

    fn faces(self) -> [u32; 3] {
        [self.l * self.w, self.w * self.h, self.l * self.h]
    }

    fn required_paper_area(self) -> u32 {
        self.faces().iter().map(|x| 2 * x).sum::<u32>() + self.smallest_area()
    }
}

impl FromStr for Present {
    type Err = ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let [l, w, h] = string
            .split('x')
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()?
            .try_into()
            .expect("Dimensions must be LxWxH of u32");

        Ok(Present { l, w, h })
    }
}

fn main() {
    let input = input(2015, 2);

    let total_paper_area: u32 = input
        .lines()
        .map(|line| Present::from_str(line).unwrap().required_paper_area())
        .sum();

    println!("Total paper required: {total_paper_area} ft²");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Present{l: 2, w: 3, h: 4}, 58)]
    #[case(Present{l: 1, w: 1, h: 10}, 43)]
    fn solves_part_1(#[case] present: Present, #[case] expected: u32) {
        assert_eq!(present.required_paper_area(), expected);
    }
}
