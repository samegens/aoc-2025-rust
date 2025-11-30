use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Range<T>
where
    T: PartialOrd + Copy,
{
    start: T,
    length: T,
}

impl<T> fmt::Debug for Range<T>
where
    T: PartialOrd + Copy + std::ops::Add<Output = T> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}..{:?})", self.start, self.start + self.length)
    }
}

impl<T> Range<T>
where
    T: PartialOrd + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default,
{
    pub fn new(start: T, length: T) -> Self {
        Range { start, length }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn length(&self) -> T {
        self.length
    }

    pub fn end(&self) -> T {
        self.start + self.length
    }

    pub fn contains(&self, n: T) -> bool {
        n >= self.start && n < self.start + self.length
    }

    /// Check if this range overlaps with another range.
    pub fn overlaps(&self, other: &Range<T>) -> bool {
        self.start < other.end() && other.start < self.end()
    }

    pub fn shifted(&self, delta: T) -> Self{
        Range { start: self.start + delta, length: self.length }
    }

    /// Split the range `self` into non-overlapping parts where the union of the parts
    /// exactly equals the range of `self`, and any part of `other` that does not overlap
    /// with `self` is excluded from the result.
    pub fn split(&self, other: &Range<T>) -> Vec<Range<T>> {
        let mut result = Vec::new();

        // No overlap, return just the `self` range as it is
        if !self.overlaps(other) {
            result.push(*self);
            return result;
        }

        // Calculate the overlap between `self` and `other`
        let overlap_start = if self.start > other.start { self.start } else { other.start };
        let overlap_end = if self.end() < other.end() { self.end() } else { other.end() };

        // Add the non-overlapping part of `self` before the overlap (if any)
        if self.start < overlap_start {
            result.push(Range::new(self.start, overlap_start - self.start));
        }

        // Add the overlapping part
        result.push(Range::new(overlap_start, overlap_end - overlap_start));

        // Add the non-overlapping part of `self` after the overlap (if any)
        if overlap_end < self.end() {
            result.push(Range::new(overlap_end, self.end() - overlap_end));
        }

        result
    }

    /// Split the range `self` based on multiple ranges provided in `others`,
    /// ensuring no overlapping ranges in the result, and excluding parts of `others`
    /// that do not overlap with `self`. The union of the resulting ranges will exactly
    /// cover `self`.
    pub fn split_on_ranges(&self, others: Vec<Range<T>>) -> Vec<Range<T>> {
        let mut result = Vec::new();

        // Start with the full `self` range
        let mut current_range = *self;

        for other in others {
            // If there's no overlap, continue to the next range
            if !current_range.overlaps(&other) {
                continue;
            }

            // Calculate the overlap
            let overlap_start = if current_range.start > other.start {
                current_range.start
            } else {
                other.start
            };
            let overlap_end = if current_range.end() < other.end() {
                current_range.end()
            } else {
                other.end()
            };

            // Add the non-overlapping part before the overlap (if any)
            if current_range.start < overlap_start {
                result.push(Range::new(current_range.start, overlap_start - current_range.start));
            }

            // Add the overlapping part
            result.push(Range::new(overlap_start, overlap_end - overlap_start));

            // Update the current range to the remaining part after the overlap
            current_range = Range::new(overlap_end, current_range.end() - overlap_end);
        }

        // Add the remaining part of `self` after processing all ranges
        if current_range.length > T::default() {
            result.push(current_range);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_no_overlap() {
        // Arrange
        let range1 = Range::new(10, 5);
        let range2 = Range::new(20, 5);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], range1);
    }

    #[test]
    fn test_split_full_overlap() {
        // Arrange
        let range1 = Range::new(10, 10); // Range from 10 to 20
        let range2 = Range::new(12, 5);  // Range from 12 to 17 (fully inside range1)

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], Range::new(10, 2));  // Non-overlapping part of range1 (before overlap)
        assert_eq!(actual[1], Range::new(12, 5));  // Overlapping part
        assert_eq!(actual[2], Range::new(17, 3));  // Non-overlapping part of range1 (after overlap)
    }

    #[test]
    fn test_split_partial_overlap() {
        // Arrange
        let range1 = Range::new(10, 10);
        let range2 = Range::new(15, 10);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Range::new(10, 5));
        assert_eq!(actual[1], Range::new(15, 5));
    }

    #[test]
    fn test_split_exact_overlap() {
        // Arrange
        let range1 = Range::new(10, 10);
        let range2 = Range::new(10, 10);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Range::new(10, 10));
    }

    #[test]
    fn test_split_non_overlapping_touching() {
        // Arrange
        let range1 = Range::new(10, 10);
        let range2 = Range::new(20, 10);

        // Act
        let actual = range1.split(&range2);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], range1);
    }

    #[test]
    fn test_split_on_ranges() {
        // Arrange
        let range_to_split = Range::new(74, 14);
        let ranges = vec![
            Range::new(77, 23),
            Range::new(45, 19),
            Range::new(64, 13)
        ];
        let expected = vec![
            Range::new(74, 3),
            Range::new(77, 11)
        ];

        // Act
        let actual = range_to_split.split_on_ranges(ranges);

        // Assert
        assert_eq!(actual, expected);
    }
}
