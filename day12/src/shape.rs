#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub id: usize,
    pub rows: [u8; 3],
}

impl Shape {
    pub fn new(id: usize, grid: &[&str]) -> Self {
        let mut rows = [0u8; 3];

        for (row_idx, row) in grid.iter().enumerate().take(3) {
            for (col_idx, ch) in row.chars().enumerate().take(3) {
                if ch == '#' {
                    rows[row_idx] |= 1 << col_idx;
                }
            }
        }

        Shape { id, rows }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_new() {
        // Arrange
        let grid = ["###", "##.", "##."];

        // Act
        let shape = Shape::new(0, &grid);

        // Assert
        assert_eq!(shape.id, 0);
        assert_eq!(shape.rows[0], 0b111);
        assert_eq!(shape.rows[1], 0b011);
        assert_eq!(shape.rows[2], 0b011);
    }
}
