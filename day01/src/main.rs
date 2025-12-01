use common::InputReader;
use std::str::Lines;
use substring::Substring;

fn main() {
    let input_reader: InputReader = InputReader::new(1);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let mut nr_zeroes = 0;
    let mut dial = 50;
    for line in lines {
        let direction_char = line.chars().nth(0).unwrap();
        let amount: i32 = line.to_string().substring(1, line.len()).parse().unwrap();
        let sign = if direction_char == 'L' { -1 } else { 1 };
        dial = (dial + 100 + sign * amount) % 100;
        if dial == 0 {
            nr_zeroes += 1;
        }
    }

    nr_zeroes
}

fn solve_part2(lines: Lines) -> i64 {
    let mut nr_zeroes = 0;
    let mut dial = 50;
    for line in lines {
        let direction_char = line.chars().nth(0).unwrap();
        let amount: i32 = line.to_string().substring(1, line.len()).parse().unwrap();
        let sign = if direction_char == 'L' { -1 } else { 1 };
        for _i in 0..amount {
            dial = (dial + 100 + sign) % 100;
            if dial == 0 {
                nr_zeroes += 1;
            }
        }
    }

    nr_zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 3;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2_with_r1000() {
        // Arrange
        let input: &str = r#"R1000
"#;
        let expected: i64 = 10;

        // Act
        let actual: i64 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2_with_not_crossing_boundaries() {
        // Arrange
        let input: &str = r#"L50
R1
L1
"#;
        let expected: i64 = 2;

        // Act
        let actual: i64 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2_with_l1000() {
        // Arrange
        let input: &str = r#"L1000
"#;
        let expected: i64 = 10;

        // Act
        let actual: i64 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = 6;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
