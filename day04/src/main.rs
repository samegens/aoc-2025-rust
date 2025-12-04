use common::{Grid, InputReader};
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(4);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> usize {
    let grid: Grid<char> = Grid::parse(lines);
    count_nr_accessable_rolls(grid)
}

fn solve_part2(_lines: Lines) -> i64 {
    0
}

fn count_nr_accessable_rolls(grid: Grid<char>) -> usize {
    let mut nr_accessable_rolls: usize = 0;
    const NEIGHBOUR_LIMIT: usize = 4;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if *grid.at(x, y).unwrap() == '@' {
                let nr_neighbours: usize = count_neighbours(&grid, x, y);
                if nr_neighbours < NEIGHBOUR_LIMIT {
                    nr_accessable_rolls += 1;
                }
            }
        }
    }

    nr_accessable_rolls
}

fn count_neighbours(grid: &Grid<char>, x: usize, y: usize) -> usize {
    let mut nr_neighbours = 0;

    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if let Some(neighbour) = grid.at(new_x, new_y)
                && *neighbour == '@'
            {
                nr_neighbours += 1;
            }
        }
    }

    nr_neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: usize = 13;

        // Act
        let actual: usize = solve_part1(INPUT.lines());

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
