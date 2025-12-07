use common::{Grid, InputReader, Point};
use std::{collections::HashMap, str::Lines};

fn main() {
    let input_reader: InputReader = InputReader::new(7);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let manifold: Grid<char> = Grid::parse(lines);
    let start: Point = get_start_point(&manifold);
    let mut visited_splitters: HashMap<Point, i64> = HashMap::new();

    follow_beam(start, &mut visited_splitters, &manifold);

    visited_splitters.len() as i64
}

fn solve_part2(lines: Lines) -> i64 {
    let manifold: Grid<char> = Grid::parse(lines);
    let start: Point = get_start_point(&manifold);
    let mut visited_splitters: HashMap<Point, i64> = HashMap::new();

    follow_beam(start, &mut visited_splitters, &manifold)
}

/// Recursively follows a beam through the manifold grid, counting the number of distinct paths.
///
/// The beam moves downward (y+1) from the start point and behaves differently based on the character encountered:
/// - `'.'`: Empty space - the beam continues moving downward
/// - `'^'`: Splitter - the beam splits into two paths going left (x-1) and right (x+1)
/// - Out of bounds: Represents the end of a path (returns 1)
///
/// # Arguments
///
/// * `start` - The current position of the beam
/// * `visited_splitters` - Cache of previously visited splitters and their total path counts
/// * `manifold` - The grid containing the beam path layout
///
/// # Returns
///
/// The total number of distinct paths from this point to the edge(s) of the grid.
/// When a splitter is encountered, the return value is the sum of paths from both branches.
/// When reaching the edge of the grid, returns 1 (one path).
///
/// # Memoization
///
/// Results are cached in `visited_splitters` to avoid recalculating paths from the same splitter.
fn follow_beam(
    start: Point,
    visited_splitters: &mut HashMap<Point, i64>,
    manifold: &Grid<char>,
) -> i64 {
    let next_point = start.translate(0, 1);
    if let Some(ch) = manifold.at_point(&next_point) {
        if *ch == '.' {
            return follow_beam(next_point, visited_splitters, manifold);
        } else if *ch == '^' {
            if !visited_splitters.contains_key(&next_point) {
                let left_point = next_point.translate(-1, 0);
                let right_point = next_point.translate(1, 0);
                let nr_paths = follow_beam(left_point, visited_splitters, manifold)
                    + follow_beam(right_point, visited_splitters, manifold);
                visited_splitters.insert(next_point, nr_paths);
                return nr_paths;
            } else {
                return visited_splitters[&next_point];
            }
        }
    }

    1
}

fn get_start_point(manifold: &Grid<char>) -> Point {
    for x in 0..manifold.width() {
        for y in 0..manifold.height() {
            if *manifold.at(x, y).unwrap() == 'S' {
                return Point::new(x as i64, y as i64);
            }
        }
    }

    Point::new(0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 21;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = 40;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_start_point() {
        // Arrange
        let manifold: Grid<char> = Grid::parse(INPUT.lines());

        // Act
        let actual: Point = get_start_point(&manifold);

        // Assert
        let expected = Point::new(7, 0);
        assert_eq!(actual, expected);
    }
}
