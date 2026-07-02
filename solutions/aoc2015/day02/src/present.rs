use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Present {
    pub l: u32,
    pub w: u32,
    pub h: u32,
}

impl Present {
    fn smallest_area(self) -> u32 {
        self.faces().into_iter().min().unwrap()
    }

    fn faces(self) -> [u32; 3] {
        [self.l * self.w, self.w * self.h, self.l * self.h]
    }

    fn volume(self) -> u32 {
        self.l * self.w * self.h
    }

    pub fn required_paper_area(self) -> u32 {
        self.faces().into_iter().map(|x| 2 * x).sum::<u32>() + self.smallest_area()
    }

    pub fn required_ribbon_length(self) -> u32 {
        let mut dimensions = [self.l, self.w, self.h];
        dimensions.sort();
        let [x, y, _] = dimensions;

        2 * (x + y) + self.volume()
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
