/// An iterator that generates sequences in breadth-first order.
///
/// Given a max_value, generates all sequences starting from length 1:
/// [0], [1], [2], ..., [max_value],
/// [0, 0], [0, 1], [0, 2], ..., [max_value, max_value],
/// [0, 0, 0], ...
pub struct SequenceGenerator {
    current: Vec<i64>,
    max_value: i64,
    started: bool,
}

impl SequenceGenerator {
    /// Creates a new SequenceGenerator with the given max value for each element.
    ///
    /// # Arguments
    ///
    /// * `max_value` - The maximum value any element in the sequence can have (inclusive)
    ///
    /// # Examples
    ///
    /// ```
    /// use common::SequenceGenerator;
    ///
    /// let mut gen = SequenceGenerator::new(2);
    /// assert_eq!(gen.next(), Some(vec![0]));
    /// assert_eq!(gen.next(), Some(vec![1]));
    /// assert_eq!(gen.next(), Some(vec![2]));
    /// assert_eq!(gen.next(), Some(vec![0, 0]));
    /// ```
    pub fn new(max_value: i64) -> Self {
        SequenceGenerator {
            current: vec![],
            max_value,
            started: false,
        }
    }

    /// Increment the current sequence like a counter in base (max_value + 1).
    /// Returns true if successful, false if we've exhausted all sequences of this length.
    fn increment(&mut self) -> bool {
        if self.current.is_empty() {
            return false;
        }

        // Try to increment from rightmost position
        for i in (0..self.current.len()).rev() {
            if self.current[i] < self.max_value {
                self.current[i] += 1;
                return true;
            } else {
                // Set to 0 and carry to the left
                self.current[i] = 0;
            }
        }

        // All digits are at max_value, we've exhausted this length
        false
    }

    /// Move to the next length by creating a new sequence of all zeros.
    fn next_length(&mut self) {
        let new_len = self.current.len() + 1;
        self.current = vec![0; new_len];
    }
}

impl Iterator for SequenceGenerator {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            // First call - start with [0]
            self.current = vec![0];
            self.started = true;
            return Some(self.current.clone());
        }

        // Try to increment current sequence
        if self.increment() {
            Some(self.current.clone())
        } else {
            // Exhausted current length, move to next length
            self.next_length();
            Some(self.current.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_generator_basic() {
        let mut gen = SequenceGenerator::new(2);

        // Length 1
        assert_eq!(gen.next(), Some(vec![0]));
        assert_eq!(gen.next(), Some(vec![1]));
        assert_eq!(gen.next(), Some(vec![2]));

        // Length 2
        assert_eq!(gen.next(), Some(vec![0, 0]));
        assert_eq!(gen.next(), Some(vec![0, 1]));
        assert_eq!(gen.next(), Some(vec![0, 2]));
        assert_eq!(gen.next(), Some(vec![1, 0]));
        assert_eq!(gen.next(), Some(vec![1, 1]));
        assert_eq!(gen.next(), Some(vec![1, 2]));
        assert_eq!(gen.next(), Some(vec![2, 0]));
        assert_eq!(gen.next(), Some(vec![2, 1]));
        assert_eq!(gen.next(), Some(vec![2, 2]));

        // Length 3 starts
        assert_eq!(gen.next(), Some(vec![0, 0, 0]));
    }

    #[test]
    fn test_sequence_generator_max_value_1() {
        let mut gen = SequenceGenerator::new(1);

        assert_eq!(gen.next(), Some(vec![0]));
        assert_eq!(gen.next(), Some(vec![1]));
        assert_eq!(gen.next(), Some(vec![0, 0]));
        assert_eq!(gen.next(), Some(vec![0, 1]));
        assert_eq!(gen.next(), Some(vec![1, 0]));
        assert_eq!(gen.next(), Some(vec![1, 1]));
        assert_eq!(gen.next(), Some(vec![0, 0, 0]));
    }

    #[test]
    fn test_sequence_generator_max_value_0() {
        let mut gen = SequenceGenerator::new(0);

        assert_eq!(gen.next(), Some(vec![0]));
        assert_eq!(gen.next(), Some(vec![0, 0]));
        assert_eq!(gen.next(), Some(vec![0, 0, 0]));
    }

    #[test]
    fn test_take_10() {
        let gen = SequenceGenerator::new(2);
        let sequences: Vec<Vec<i64>> = gen.take(10).collect();

        assert_eq!(sequences.len(), 10);
        assert_eq!(sequences[0], vec![0]);
        assert_eq!(sequences[9], vec![2, 0]);
    }
}
