mod checker;
mod parser;
mod puzzle_input;
mod region;
mod shape;

use checker::count_fittable_regions;
use common::InputReader;
use parser::parse_input;
use std::str::Lines;

fn main() {
    let input_reader: InputReader = InputReader::new(12);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let puzzle = parse_input(lines);
    count_fittable_regions(&puzzle) as i64
}

fn solve_part2(_lines: Lines) -> i64 {
    // Just press the link.
    0
}

#[cfg(test)]
mod tests {
    // No tests. To 'solve' the puzzle I used a heuristic that doesn't work
    // on the sample input.
}
