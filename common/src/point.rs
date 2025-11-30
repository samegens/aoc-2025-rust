#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn translate(&self, dx: i64, dy: i64) -> Self {
        Point::new(self.x + dx, self.y + dy)
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn manhattan_distance(&self, other: &Point) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    /// Returns an iterator over the surrounding points of this point.
    pub fn adjacent_points<'a>(&'a self) -> AdjacentPoints<'a> {
        AdjacentPoints {
            center: self,
            index: 0,
        }
    }
}

/// Custom iterator to iterate over the adjacent points of a Point.
pub struct AdjacentPoints<'a> {
    center: &'a Point,
    index: usize,
}

impl<'a> Iterator for AdjacentPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Define all 8 possible offsets around a point in clockwise order
        let offsets = [
            (-1, -1), // Top-left
            (0, -1),  // Top
            (1, -1),  // Top-right
            (-1, 0),  // Left
            (1, 0),   // Right
            (-1, 1),  // Bottom-left
            (0, 1),   // Bottom
            (1, 1),   // Bottom-right
        ];

        if self.index < offsets.len() {
            let (dx, dy) = offsets[self.index];
            self.index += 1;
            Some(Point::new(self.center.x + dx, self.center.y + dy))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        // Arrange
        let point = Point::new(3, 4);
        let expected = Point::new(5, 7);

        // Act

        let actual = point.translate(2, 3);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_distance_to() {
        // Arrange
        let point1 = Point::new(1, 2);
        let point2 = Point::new(4, 6);
        let expected = 5.0;

        // Act
        let actual = point1.distance_to(&point2);

        // Assert
        assert!((actual - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_manhattan_distance() {
        // Arrange
        let point1 = Point::new(1, 2);
        let point2 = Point::new(4, 6);
        let expected = 7;

        // Act
        let actual = point1.manhattan_distance(&point2);

        // Assert
        assert_eq!(actual, expected);
    }
}
