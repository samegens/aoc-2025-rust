use common::InputReader;
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(6);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let mut sum: i64 = 0;
    let problems: Vec<Vec<&str>> = lines
        .map(|line: &str| line.split_ascii_whitespace().collect())
        .collect();
    let operands = problems.last().unwrap();

    for x in 0..problems[0].len() {
        let operand = operands[x];
        let mut result = if operand == "*" { 1 } else { 0 };
        for y in 0..problems.len() - 1 {
            let number: i64 = problems[y][x].parse().unwrap();
            if operand == "*" {
                result *= number;
            } else {
                result += number;
            }
        }
        sum += result;
    }

    sum
}

fn solve_part2(_: Lines) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 4277556;

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
}
