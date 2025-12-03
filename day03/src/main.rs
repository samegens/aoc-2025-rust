use common::InputReader;
use std::str::Lines;
use substring::Substring;

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

fn solve_part2(lines: Lines) -> i64 {
    lines
        .map(|line: &str| get_max_joltage(line.to_string(), 12))
        .sum()
}

fn get_max_joltage(bank: String, nr_batteries: usize) -> i64 {
    // Algorithm:
    // When turning on 12 batteries, we can only look at the first
    // len - 11 batteries, since we have to be able to select 11 more batteries.
    // From these 12, we select the battery with the highest joltage.
    // Then we look at the batteries at the right from the selected battery,
    // keeping in mind that we can't use the last 10 batteries, since we
    // have to select 10 more batteries. Etc.

    let digits: Vec<i64> = bank.bytes().map(|b| (b - b'0') as i64).collect();

    let mut joltage: i64 = 0;
    let mut pos = 0;
    for i in 0..nr_batteries {
        let joltage_digit = digits
            .iter()
            .skip(pos)
            .take(digits.len() - (nr_batteries - i - 1) - pos)
            .max()
            .unwrap();
        let joltage_char = char::from_digit(*joltage_digit as u32, 10).unwrap();
        pos = pos + bank.substring(pos, bank.len()).find(joltage_char).unwrap() + 1;
        joltage = joltage * 10 + *joltage_digit;
    }

    joltage
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
        let expected: i64 = 3121910778619;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    #[case("987654321111111", 12, 987654321111)]
    #[case("811111111111119", 12, 811111111119)]
    #[case("234234234234278", 12, 434234234278)]
    #[case("818181911112111", 12, 888911112111)]
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
