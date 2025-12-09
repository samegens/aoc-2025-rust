use crate::Point;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Rect {
    pub top: i64,
    pub left: i64,
    pub bottom: i64,
    pub right: i64,
}

impl Rect {
    pub fn new(p1: &Point, p2: &Point) -> Rect {
        Rect {
            top: std::cmp::min(p1.y, p2.y),
            left: std::cmp::min(p1.x, p2.x),
            bottom: std::cmp::max(p1.y, p2.y),
            right: std::cmp::max(p1.x, p2.x),
        }
    }

    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.left && p.x <= self.right && p.y >= self.top && p.y <= self.bottom
    }

    pub fn strictly_contains(&self, p: &Point) -> bool {
        p.x > self.left && p.x < self.right && p.y > self.top && p.y < self.bottom
    }

    pub fn is_corner(&self, p: &Point) -> bool {
        (p.x == self.left || p.x == self.right) && (p.y == self.top || p.y == self.bottom)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Arrange
        let p1 = Point::new(3, 4);
        let p2 = Point::new(1, 2);

        // Act
        let actual = Rect::new(&p1, &p2);

        // Assert
        let expected = Rect {
            top: 2,
            left: 1,
            bottom: 4,
            right: 3,
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: 0}, true)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: -1, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 1, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: -1}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: 1}, false)]
    fn test_contains(#[case] rect: Rect, #[case] point: Point, #[case] expected: bool) {
        // Act
        let actual = rect.contains(&point);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: -1, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 1, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: -1}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: 1}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 2, right: 2}, Point{x: 1, y: 1}, true)]
    fn test_strictly_contains(#[case] rect: Rect, #[case] point: Point, #[case] expected: bool) {
        // Act
        let actual = rect.strictly_contains(&point);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: 0}, true)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: -1, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 1, y: 0}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: -1}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 0, right: 0}, Point{x: 0, y: 1}, false)]
    #[case(Rect {top: 0, left: 0, bottom: 1, right: 1}, Point{x: 1, y: 1}, true)]
    #[case(Rect {top: 0, left: 0, bottom: 2, right: 2}, Point{x: 1, y: 1}, false)]
    fn test_is_corner(#[case] rect: Rect, #[case] point: Point, #[case] expected: bool) {
        // Act
        let actual = rect.is_corner(&point);

        // Assert
        assert_eq!(actual, expected);
    }
}
