use common::{Grid, InputReader};
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

fn solve_part2(lines: Lines) -> i64 {
    // Algorithm:
    // The puzzles describes a right-to-left way of parsing but that is not actually needed.
    // The numbers are simply built up top-to-bottom, with numbers on earlier lines being more significant.
    // So we can easily built a number by parsing all chars in a column (except for the last line) in the
    // same way we would parse a left-to-right number: by multiplying by 10 before adding the next digit.
    // We do this in a left to right way, where every column that has an operand determines the start of
    // a new problem. Between columns we apply the operand that defined the start of the problem.
    let worksheet: Grid<char> = Grid::parse(lines);
    let mut sum: i64 = 0;
    let operand_y: usize = worksheet.height() - 1;
    let mut current_operand = worksheet.at(0, operand_y).unwrap();

    let mut current_result: i64 = 0;
    for x in 0..worksheet.width() {
        let operand_of_current_column = worksheet.at(x, operand_y).unwrap();
        if *operand_of_current_column != ' ' {
            // Done with the previous problem, we need to add the final result to the global sum and
            // reset for the next problem.
            current_operand = operand_of_current_column;
            sum += current_result;
            current_result = if *current_operand == '*' { 1 } else { 0 };
        }

        if let Some(current_value) = get_value_from_column(&worksheet, x) {
            if *current_operand == '*' {
                current_result *= current_value;
            } else {
                current_result += current_value;
            }
        }
    }

    // We didn't finish the last problem, so we do that now.
    sum += current_result;

    sum
}

fn get_value_from_column(worksheet: &Grid<char>, x: usize) -> Option<i64> {
    let mut current_value: i64 = 0;
    let mut is_column_with_value = false;

    for y in 0..worksheet.height() - 1 {
        let optional_number = worksheet.at(x, y).unwrap();
        if *optional_number != ' ' {
            current_value = current_value * 10 + optional_number.to_digit(10).unwrap() as i64;
            is_column_with_value = true;
        }
    }

    if is_column_with_value {
        Some(current_value)
    } else {
        None
    }
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
        let expected: i64 = 3263827;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
