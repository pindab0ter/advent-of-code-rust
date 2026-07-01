use aoc_client::input;

fn main() {
    let instructions = input(2015, 1);

    let floor = find_floor(&instructions);
    println!("The instructions take Santa to floor {floor}");

    let instruction_index = find_when_entering_basement(&instructions);
    println!("The basement was eneterd at instruction #{instruction_index}")
}

fn find_floor(instructions: &str) -> i32 {
    instructions.bytes().fold(0, |acc, instruction| {
        acc + match instruction {
            b'(' => 1,
            b')' => -1,
            _ => panic!("Invalid character"),
        }
    })
}

fn find_when_entering_basement(instructions: &str) -> i32 {
     instructions
        .bytes()
        .scan(0, |floor, instruction| {
            *floor += match instruction {
                b'(' => 1,
                b')' => -1,
                _ => panic!("Invalid character"),
            };
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .map(|index| index + 1)
        .unwrap() as i32
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

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn finds_when_the_basement_is_entered(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(find_when_entering_basement(input), expected);
    }
}
