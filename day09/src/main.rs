use common::{InputReader, Point};
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(9);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let points: Vec<Point> = lines.map(|line| parse_line(line)).collect();
    generate_surface_areas(&points).into_iter().max().unwrap()
}

fn solve_part2(_lines: Lines) -> i64 {
    0
}

fn parse_line(line: &str) -> Point {
    let parts: Vec<i64> = line
        .split(',')
        .map(|part| part.parse::<i64>().unwrap())
        .collect();
    Point::new(parts[0], parts[1])
}

fn generate_surface_areas(points: &Vec<Point>) -> Vec<i64> {
    let mut surface_areas: Vec<i64> = vec![];

    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let width = (points[j].x - points[i].x).abs() + 1;
            let height = (points[j].y - points[i].y).abs() + 1;
            let surface_area = width * height;
            surface_areas.push(surface_area);
        }
    }

    surface_areas
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let expected: i64 = 50;

        // Act
        let actual: i64 = solve_part1(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let expected: i64 = ;

        // Act
        let actual: i64 = solve_part2(INPUT.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line() {
        // Arrange
        let line = "7,1";

        // Act
        let actual: Point = parse_line(line);

        // Assert
        let expected = Point::new(7, 1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_generate_surface_areas() {
        // Arrange
        let points: Vec<Point> = vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)];

        // Act
        let actual: Vec<i64> = generate_surface_areas(&points);

        // Assert
        let expected: Vec<i64> = vec![4, 9, 4];
        assert_eq!(actual, expected);
    }
}
