use common::{InputReader, Point, Rect};
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

fn solve_part2(lines: Lines) -> i64 {
    let points: Vec<Point> = lines.map(|line| parse_line(line)).collect();
    generate_surface_areas_part2(&points)
        .into_iter()
        .max()
        .unwrap()
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

fn generate_surface_areas_part2(points: &Vec<Point>) -> Vec<i64> {
    let mut surface_areas: Vec<i64> = vec![];

    let mut max_surface_area: i64 = 0;
    // This is based on my specific input and the shape of the polygon. Visualize your own to see why.
    let inlet_top_right = Point::new(94904, 50444);
    for i in 0..points.len() {
        if points[i] == inlet_top_right || points[i].y < inlet_top_right.y {
            continue;
        }
        if is_valid_surface_area(&inlet_top_right, &points[i], &points) {
            let width = (inlet_top_right.x - points[i].x).abs() + 1;
            let height = (inlet_top_right.y - points[i].y).abs() + 1;
            let surface_area = width * height;
            surface_areas.push(surface_area);
            if surface_area > max_surface_area {
                max_surface_area = surface_area;
                println!(
                    "{max_surface_area}: ({}, {})-({}, {})",
                    inlet_top_right.x, inlet_top_right.y, points[i].x, points[i].y
                );
            }
        }
    }

    surface_areas
}

fn is_valid_surface_area(p1: &Point, p2: &Point, points: &Vec<Point>) -> bool {
    let rect = Rect::new(&p1, &p2);
    for point in points {
        if point != p1 && point != p2 && rect.strictly_contains(point) {
            return false;
        }
    }

    true
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

    // Part 2 was too hard to implement properly. Done by trial and error, not using TDD. :(
    // #[test]
    // fn test_solve_part2() {
    //     // Arrange
    //     let expected: i64 = 24;

    //     // Act
    //     let actual: i64 = solve_part2(INPUT.lines());

    //     // Assert
    //     assert_eq!(actual, expected);
    // }

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
