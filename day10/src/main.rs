mod machine;

use common::{InputReader, SequenceGenerator};
use machine::Machine;
use regex::Regex;
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(10);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    lines
        .map(|line| find_min_number_of_button_presses(&mut parse_line(line)))
        .sum()
}

fn solve_part2(_lines: Lines) -> i64 {
    0
}

fn parse_line(line: &str) -> Machine {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    let regex = Regex::new(r"\[(.+)\] ((?:\(\d+(?:,\d+)*\) )+)\{(\d+(?:,\d+)*)\}").unwrap();
    let captures = regex.captures(line).unwrap();
    let light_diagram: Vec<bool> = parse_light_diagram(&captures[1]);
    let wiring_schematics: Vec<Vec<i64>> = parse_wiring_schematics(&captures[2]);
    let joltage_requirements: Vec<i64> = parse_numbers(&captures[3]);

    Machine::new(&light_diagram, &wiring_schematics, &joltage_requirements)
}

fn parse_light_diagram(s: &str) -> Vec<bool> {
    s.chars().map(|ch| ch == '#').collect()
}

fn parse_wiring_schematics(s: &str) -> Vec<Vec<i64>> {
    let regex = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
    regex
        .captures_iter(s)
        .map(|caps| parse_numbers(&caps[1]))
        .collect()
}

fn parse_numbers(s: &str) -> Vec<i64> {
    s.split(',')
        .map(|part| part.parse::<i64>().unwrap())
        .collect()
}

fn find_min_number_of_button_presses(machine: &mut Machine) -> i64 {
    let mut min_nr_button_presses = i64::MAX;
    let nr_buttons = machine.wiring_schematics().len();
    for variation in 0..1 << nr_buttons {
        machine.reset_lights();
        let mut nr_buttons_pressed = 0;
        for button_index in 0..nr_buttons {
            let bit = 1 << button_index;
            if variation & bit != 0 {
                nr_buttons_pressed += 1;
                if machine.push_button(button_index as i64) {
                    if nr_buttons_pressed < min_nr_button_presses {
                        min_nr_button_presses = nr_buttons_pressed;
                        continue;
                    }
                }
            }
        }
    }

    min_nr_button_presses
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 7;

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

    #[test]
    fn test_parse_line() {
        // Arrange
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

        // Act
        let actual: Machine = parse_line(line);

        // Assert
        let expected_light_diagram: Vec<bool> = vec![false, true, true, false];
        let expected_wiring_schematics: Vec<Vec<i64>> = vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ];
        let expected_joltage_requirements: Vec<i64> = vec![3, 5, 4, 7];
        let expected = Machine::new(
            &expected_light_diagram,
            &expected_wiring_schematics,
            &expected_joltage_requirements,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_min_number_of_button_presses() {
        // Arrange
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let mut machine = parse_line(line);

        // Act
        let actual: i64 = find_min_number_of_button_presses(&mut machine);

        // Assert
        let expected: i64 = 2;
        assert_eq!(actual, expected);
    }
}
