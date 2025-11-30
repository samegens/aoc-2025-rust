use std::str::Lines;

pub fn split_into_blocks<'a>(lines: Lines<'a>) -> Vec<Vec<&'a str>> {
    let mut blocks: Vec<Vec<&str>> = Vec::new();
    let mut current_block: Vec<&str> = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            if !current_block.is_empty() {
                blocks.push(current_block);
                current_block = Vec::new();
            }
        } else {
            current_block.push(line);
        }
    }

    // Push the last block if it has content
    if !current_block.is_empty() {
        blocks.push(current_block);
    }

    blocks
}

pub fn parse_numbers_from_string(string: &str) -> Vec<i64> {
    string
        .split_whitespace()
        .map(|s| s.parse().unwrap()) // Parse each number into i64
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_blocks() {
        // Arrange
        let input = r#"This is line 1
This is line 2

This is line 3
This is line 4


This is line 5

"#;
        let lines = input.lines();

        // Act
        let result = split_into_blocks(lines);

        // Assert
        let expected = vec![
            vec!["This is line 1", "This is line 2"],
            vec!["This is line 3", "This is line 4"],
            vec!["This is line 5"],
        ];

        assert_eq!(result, expected);
    }
}
