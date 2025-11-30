use std::str::Lines;

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    data: Vec<Vec<T>>
}

impl<T> Grid<T> where T: From<char>, // Ensures T can be constructed from a char
{
    pub fn parse(lines: Lines) -> Self {
        let data = lines
            .map(|line| line.chars().map(T::from).collect())
            .collect();
        Grid { data }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        self.data
            .get(y)
            .and_then(|row| row.get(x))
    }

    pub fn width(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }

    pub fn height(&self) -> usize {
        self.data.len() // Number of rows in the grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // Arrange
        let input = "abcd\nefgh\nijkl\n";

        // Act
        let grid: Grid<char> = Grid::parse(input.lines());

        // Assert
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.width(), 4);

        assert_eq!(grid.at(0, 0), Some(&'a'));
        assert_eq!(grid.at(3, 2), Some(&'l'));
    }

    #[test]
    fn test_at_out_of_bounds() {
        // Arrange
        let input = "abcd\nefgh\nijkl\n";
        let grid: Grid<char> = Grid::parse(input.lines());
        let expected = None;

        // Act
        let actual: Option<&char> = grid.at(4, 0);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_width_and_height() {
        let input = "123\n456\n789\n";
        let grid: Grid<char> = Grid::parse(input.lines());

        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_empty_grid() {
        // Arrange
        let input = "";

        // Act
        let grid: Grid<char> = Grid::parse(input.lines());

        // Assert
        assert_eq!(grid.width(), 0);
        assert_eq!(grid.height(), 0);
        assert_eq!(grid.at(0, 0), None);
    }
}
