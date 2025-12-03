use common::InputReader;
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(3);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    lines
        .map(|line: &str| get_max_joltage(line.to_string(), 2))
        .sum()
}

fn solve_part2(_: Lines) -> i64 {
    0
}

fn get_max_joltage(bank: String, nr_batteries: usize) -> i64 {
    let digits: Vec<u32> = bank.bytes().map(|b| (b - b'0') as u32).collect();
    let first_joltage_digit = digits
        .iter()
        .take(digits.len() - (nr_batteries - 1))
        .max()
        .unwrap();
    let first_joltage_char = char::from_digit(*first_joltage_digit, 10).unwrap();
    let first_joltage_digit_pos = bank.find(first_joltage_char).unwrap();
    let second_joltage_digit = digits
        .iter()
        .skip(first_joltage_digit_pos + 1)
        .max()
        .unwrap();
    (first_joltage_digit * 10 + second_joltage_digit) as i64
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 357;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = 0;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    fn test_get_max_joltage(
        #[case] bank: &str,
        #[case] nr_batteries: usize,
        #[case] expected: i64,
    ) {
        // Act
        let actual = get_max_joltage(bank.to_string(), nr_batteries);

        // Assert
        assert_eq!(actual, expected);
    }
}
