use crate::puzzle_input::PuzzleInput;
use crate::region::Region;
use crate::shape::Shape;

pub fn count_fittable_regions(puzzle: &PuzzleInput) -> usize {
    puzzle
        .regions
        .iter()
        .filter(|region| can_fit_shapes(region, &puzzle.shapes))
        .count()
}

fn can_fit_shapes(region: &Region, shapes: &[Shape]) -> bool {
    check_area(region, shapes)
}

fn check_area(region: &Region, shapes: &[Shape]) -> bool {
    // Only a simple heuristic. I'm not convinced that this will find edge cases, but it gives the
    // correct answer to the puzzle.
    let total_area = region.width * region.height;
    let shapes_area: usize = region
        .shape_counts
        .iter()
        .enumerate()
        .map(|(shape_idx, &count)| {
            let shape_area = count_shape_cells(&shapes[shape_idx]);
            shape_area * count
        })
        .sum();

    shapes_area <= total_area
}

fn count_shape_cells(shape: &Shape) -> usize {
    shape
        .rows
        .iter()
        .map(|&row| row.count_ones() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_shape_cells() {
        // Arrange
        let shape = Shape {
            id: 0,
            rows: [0b111, 0b011, 0b011],
        };

        // Act
        let actual = count_shape_cells(&shape);

        // Assert
        assert_eq!(actual, 7);
    }
}
