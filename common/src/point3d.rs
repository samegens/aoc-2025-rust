#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
pub struct Point3d {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3d {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Point3d { x, y, z }
    }

    pub fn distance_to(&self, other: &Point3d) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        let dz = (other.z - self.z) as f64;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to() {
        // Arrange
        let point1 = Point3d::new(1, 2, 3);
        let point2 = Point3d::new(3, 4, 4);
        let expected = 3.0;

        // Act
        let actual = point1.distance_to(&point2);

        // Assert
        assert!((actual - expected).abs() < f64::EPSILON);
    }
}
