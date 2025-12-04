use common::{Grid, InputReader, Point};
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(4);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> usize {
    let grid: Grid<char> = Grid::parse(lines);
    get_accessable_rolls(&grid).len()
}

fn solve_part2(lines: Lines) -> usize {
    let mut grid: Grid<char> = Grid::parse(lines);
    let mut nr_rolls_removed = 0;

    loop {
        let accessable_rolls = get_accessable_rolls(&grid);
        if accessable_rolls.len() == 0 {
            break;
        }
        remove_positions(&mut grid, &accessable_rolls);
        nr_rolls_removed += accessable_rolls.len();
    }

    nr_rolls_removed
}

fn get_accessable_rolls(grid: &Grid<char>) -> Vec<Point> {
    let mut accessable_rolls: Vec<Point> = vec![];
    const NEIGHBOUR_LIMIT: usize = 4;

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if *grid.at(x, y).unwrap() == '@' {
                let nr_neighbours: usize = count_neighbours(&grid, x, y);
                if nr_neighbours < NEIGHBOUR_LIMIT {
                    accessable_rolls.push(Point::new(x as i64, y as i64));
                }
            }
        }
    }

    accessable_rolls
}

fn remove_positions(grid: &mut Grid<char>, positions: &Vec<Point>) {
    for pos in positions {
        grid.remove_at(pos, '.');
    }
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
        let expected: usize = 43;

        // Act
        let actual: usize = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }
}
