use aoc_client::input;

fn main() {
    let instructions = input(2015, 1);

    let result: i32 = find_floor(&instructions);
    println!("The instructions take Santa to floor {result}");
}

fn find_floor(instructions: &str) -> i32 {
    instructions.bytes().fold(0, |acc, char| {
        acc + match char {
            b'(' => 1,
            b')' => -1,
            _ => panic!("Invalid character"),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn finds_the_correct_floor(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(find_floor(input), expected);
    }
}
