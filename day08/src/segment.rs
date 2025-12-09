use common::Point3d;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
pub struct Segment {
    pub start: Point3d,
    pub end: Point3d,
    pub squared_length: i64,
}

impl Segment {
    pub fn new(start: &Point3d, end: &Point3d) -> Segment {
        Segment {
            start: start.clone(),
            end: end.clone(),
            squared_length: Self::get_squared_length(start, end),
        }
    }

    fn get_squared_length(start: &Point3d, end: &Point3d) -> i64 {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dz = end.z - start.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Act
        let start = Point3d { x: 1, y: 2, z: 3 };
        let end = Point3d { x: 2, y: 4, z: 6 };
        let actual = Segment::new(&start, &end);

        // Assert
        assert_eq!(actual.squared_length, 14);
        assert_eq!(actual.start, start);
        assert_eq!(actual.end, end);
    }
}
