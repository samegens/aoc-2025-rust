use common::{InputReader, Range};
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(5);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let (fresh_ranges, ingredients) = parse_lines(lines);
    ingredients
        .into_iter()
        .filter(|ingredient: &i64| is_ingredient_fresh(*ingredient, &fresh_ranges))
        .count() as i64
}

fn solve_part2(lines: Lines) -> i64 {
    let (mut fresh_ranges, _ingredients) = parse_lines(lines);

    fresh_ranges.sort_by(|a, b| a.start().cmp(&b.start()));
    for i in 0..fresh_ranges.len() - 1 {
        let current_range = fresh_ranges[i];

        if current_range.length() == 0 {
            continue;
        }

        for j in i + 1..fresh_ranges.len() {
            let other_range = fresh_ranges[j];

            if other_range.length() == 0 {
                continue;
            }

            if !current_range.overlaps(&other_range) {
                // Since the list is sorted by start, we won't find overlap with other ranges.
                break;
            }

            // If this range fully contains the next range, we want to ignore the next range,
            // otherwise we might possibly ignore a part of this range that extends beyond
            // the end of the next range.
            if current_range.contains_range(&other_range) {
                fresh_ranges[j] = Range::new(current_range.start(), 0);
                continue;
            }

            let new_start = current_range.start();
            let new_length = other_range.start() - current_range.start();
            fresh_ranges[i] = Range::new(new_start, new_length);

            // Since the list is sorted by start, we won't find overlap with other ranges.
            break;
        }
    }

    fresh_ranges
        .into_iter()
        .map(|fresh_range| fresh_range.length())
        .sum::<i64>() as i64
}

fn parse_lines(lines: Lines) -> (Vec<Range<i64>>, Vec<i64>) {
    let mut fresh_ranges: Vec<Range<i64>> = vec![];
    let mut ingredients: Vec<i64> = vec![];

    let mut is_parsing_fresh = true;

    for line in lines {
        if line.len() == 0 {
            is_parsing_fresh = false;
        } else if is_parsing_fresh {
            let range: Range<i64> = parse_range(line);
            fresh_ranges.push(range);
        } else {
            let ingredient: i64 = line.parse().unwrap();
            ingredients.push(ingredient);
        }
    }

    (fresh_ranges, ingredients)
}

fn parse_range(line: &str) -> Range<i64> {
    let parts: Vec<&str> = line.split('-').collect();
    let start: i64 = parts[0].parse().unwrap();
    let end: i64 = parts[1].parse().unwrap();
    Range::new(start, end - start + 1)
}

fn is_ingredient_fresh(ingredient: i64, fresh_ranges: &Vec<Range<i64>>) -> bool {
    for fresh_range in fresh_ranges {
        if fresh_range.contains(ingredient) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    static INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
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
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = 14;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2_with_complex_ranges() {
        // Arrange
        let input: &str = r#"3-5
4-4

"#;
        let expected: i64 = 3;

        // Act
        let actual: i64 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2_with_more_complex_ranges() {
        // Arrange
        let input: &str = r#"3-5
4-4
4-4

"#;
        let expected: i64 = 3;

        // Act
        let actual: i64 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_input() {
        // Act
        let (actual_ranges, actual_ingredients) = parse_lines(INPUT.lines());

        // Assert
        let expected_ranges: Vec<Range<i64>> = vec![
            Range::new(3, 3),
            Range::new(10, 5),
            Range::new(16, 5),
            Range::new(12, 7),
        ];
        assert_eq!(actual_ranges, expected_ranges);

        let expected_ingredients: Vec<i64> = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(actual_ingredients, expected_ingredients);
    }

    #[rstest]
    #[case(0, vec![Range::new(1, 2)], false)]
    #[case(1, vec![Range::new(1, 2)], true)]
    #[case(2, vec![Range::new(1, 2)], true)]
    #[case(3, vec![Range::new(1, 2)], false)]
    #[case(4, vec![Range::new(1, 2), Range::new(4, 1)], true)]
    fn test_is_ingredient_fresh(
        #[case] ingredient: i64,
        #[case] fresh_ranges: Vec<Range<i64>>,
        #[case] expected: bool,
    ) {
        // Act
        let actual: bool = is_ingredient_fresh(ingredient, &fresh_ranges);

        // Assert
        assert_eq!(actual, expected);
    }
}
