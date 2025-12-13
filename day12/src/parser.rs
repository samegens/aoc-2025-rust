use crate::puzzle_input::PuzzleInput;
use crate::region::Region;
use crate::shape::Shape;
use std::str::Lines;

pub fn parse_input(lines: Lines) -> PuzzleInput {
    let lines_vec: Vec<&str> = lines.collect();
    let shapes = parse_shapes(&lines_vec);
    let regions = parse_regions(&lines_vec);

    PuzzleInput { shapes, regions }
}

fn parse_shapes(lines: &[&str]) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if is_shape_header(lines[i]) {
            let shape_id = parse_shape_id(lines[i]);
            let grid = &lines[i + 1..i + 4];
            shapes.push(Shape::new(shape_id, grid));
            i += 4;
        } else {
            i += 1;
        }
    }

    shapes
}

fn parse_regions(lines: &[&str]) -> Vec<Region> {
    lines
        .iter()
        .filter(|line| is_region_line(line))
        .map(|line| parse_region_line(line))
        .collect()
}

fn is_shape_header(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.contains(':') && !trimmed.contains('x')
}

fn is_region_line(line: &str) -> bool {
    line.trim().contains('x')
}

fn parse_shape_id(line: &str) -> usize {
    line.trim().trim_end_matches(':').parse().unwrap()
}

fn parse_region_line(line: &str) -> Region {
    let parts: Vec<&str> = line.split(':').collect();
    let (width, height) = parse_dimensions(parts[0]);
    let shape_counts = parse_shape_counts(parts[1]);

    Region::new(width, height, shape_counts)
}

fn parse_dimensions(dim_str: &str) -> (usize, usize) {
    let dims: Vec<&str> = dim_str.trim().split('x').collect();
    let width = dims[0].parse().unwrap();
    let height = dims[1].parse().unwrap();
    (width, height)
}

fn parse_shape_counts(counts_str: &str) -> Vec<usize> {
    counts_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        // Arrange
        let input = r#"0:
###
##.
##.

1:
###
##.
.##

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2"#;

        // Act
        let result = parse_input(input.lines());

        // Assert
        assert_eq!(result.shapes.len(), 2);
        assert_eq!(result.shapes[0].id, 0);
        assert_eq!(result.shapes[1].id, 1);
        assert_eq!(result.regions.len(), 2);
        assert_eq!(result.regions[0].width, 4);
        assert_eq!(result.regions[0].height, 4);
        assert_eq!(result.regions[0].shape_counts, vec![0, 0, 0, 0, 2, 0]);
        assert_eq!(result.regions[1].width, 12);
        assert_eq!(result.regions[1].height, 5);
        assert_eq!(result.regions[1].shape_counts, vec![1, 0, 1, 0, 2, 2]);
    }
}
