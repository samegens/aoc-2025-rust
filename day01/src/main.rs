use std::str::Lines;
use common::InputReader;

fn main() {
    let input_reader: InputReader = InputReader::new(8);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(_: Lines) -> i64 {
    0
}

fn solve_part2(_: Lines) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#""#;

    #[test]
    fn test_solve_part1()
    {
        // Arrange
        let expected: i64 = 0;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2()
    {
        // Arrange
        let expected: i64 = 0;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
