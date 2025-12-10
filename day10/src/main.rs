mod machine;

use common::InputReader;
use good_lp::*;
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

fn solve_part2(lines: Lines) -> i64 {
    lines
        .map(|line| find_min_number_of_button_presses_part2(&parse_line(line)))
        .sum()
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

fn find_min_number_of_button_presses_part2(machine: &Machine) -> i64 {
    let wiring_schematics: &[Vec<i64>] = machine.wiring_schematics();
    let target_joltages: &[i64] = machine.joltage_requirements();

    // Create variables storage
    let mut vars = ProblemVariables::new();

    // Create variables for each button (how many times pressed)
    let button_vars: Vec<Variable> = (0..wiring_schematics.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    // Build objective: minimize total button presses
    let objective: Expression = button_vars.iter().sum();

    // Start building problem
    let mut problem = vars.minimise(objective).using(default_solver);

    // For each counter, sum of button presses must equal target
    for (joltage_idx, &target_joltage) in target_joltages.iter().enumerate() {
        let constraint: Expression = wiring_schematics
            .iter()
            .enumerate()
            .filter(|(_button_idx, button)| button.contains(&(joltage_idx as i64)))
            .map(|(button_idx, _button)| button_vars[button_idx])
            .sum();

        problem = problem.with(constraint.eq(target_joltage as i32));
    }

    // Solve
    match problem.solve() {
        Ok(solution) => button_vars
            .iter()
            .map(|&variable| solution.value(variable).round() as i64)
            .sum::<i64>(),
        Err(e) => {
            panic!("No solution found: {:?}", e);
        }
    }
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
        let expected: i64 = 10 + 12 + 11;

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
