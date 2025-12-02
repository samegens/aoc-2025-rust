use common::{InputReader, Range};
use substring::Substring;

fn main() {
    let input_reader: InputReader = InputReader::new(2);
    println!("Part 1: {}", solve_part1(input_reader.input()));
    println!("Part 2: {}", solve_part2(input_reader.input()));
}

fn solve_part1(line: String) -> i64 {
    parse_ranges(&line)
        .into_iter()
        .map(|range: Range<i64>| get_invalid_ids_part1(range).iter().sum::<i64>())
        .sum()
}

fn solve_part2(_: String) -> i64 {
    0
}

fn get_invalid_ids_part1(range: Range<i64>) -> Vec<i64> {
    (range.start()..range.end() + 1)
        .filter(|id: &i64| !is_valid_id_part1(*id))
        .collect()
}

fn is_valid_id_part1(id: i64) -> bool {
    let id_string: String = id.to_string();
    if id_string.len() % 2 == 1 {
        return true;
    }

    let first_half = id_string.substring(0, id_string.len() / 2);
    let second_half = id_string.substring(id_string.len() / 2, id_string.len());
    if first_half == second_half {
        return false;
    }

    true
}

fn parse_ranges(ranges_text: &str) -> Vec<Range<i64>> {
    ranges_text
        .split(',')
        .map(|r: &str| parse_range(r))
        .collect()
}

fn parse_range(range_text: &str) -> Range<i64> {
    let numbers: Vec<i64> = range_text
        .split('-')
        .map(|s: &str| str::parse::<i64>(s).unwrap())
        .collect();
    let length = numbers[1] - numbers[0] + 1;
    Range::new(numbers[0], length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    static INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 1227775554;

        // Act
        let actual: i64 = solve_part1(INPUT.to_string());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = 4174379265;

        // Act
        let actual: i64 = solve_part2(INPUT.to_string());

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(1, true)]
    #[case(12, true)]
    #[case(11, false)]
    #[case(6464, false)]
    #[case(123123, false)]
    fn test_is_valid_id(#[case] id: i64, #[case] expected_is_valid: bool) {
        // Arrange

        // Act
        let actual: bool = is_valid_id_part1(id);

        // Assert
        assert_eq!(actual, expected_is_valid);
    }

    #[rstest]
    #[case("2-3", vec![Range::new(2, 2)])]
    #[case("2-3,4-6", vec![Range::new(2, 2), Range::new(4, 3)])]
    fn test_parse_ranges(#[case] ranges_text: &str, #[case] expected: Vec<Range<i64>>) {
        // Act
        let actual = parse_ranges(ranges_text);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Range::new(11, 12), vec![11, 22])]
    fn test_get_invalid_ids(#[case] range: Range<i64>, #[case] expected: Vec<i64>) {
        // Act
        let actual = get_invalid_ids_part1(range);

        // Assert
        assert_eq!(actual, expected);
    }
}
