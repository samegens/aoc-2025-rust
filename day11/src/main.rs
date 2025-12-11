use common::{DirectedGraph, InputReader};
use std::{collections::HashMap, str::Lines};

fn main() {
    let input_reader: InputReader = InputReader::new(11);
    println!("Part 1: {}", solve_part1(input_reader.lines()));
    println!("Part 2: {}", solve_part2(input_reader.lines()));
}

fn solve_part1(lines: Lines) -> i64 {
    let graph: DirectedGraph = parse_lines(lines);
    graph.count_possible_paths("you", "out")
}

fn parse_lines(lines: Lines<'_>) -> DirectedGraph {
    let mut graph = DirectedGraph::new();

    for line in lines {
        if let Some((from, to_nodes)) = line.split_once(": ") {
            for to in to_nodes.split_whitespace() {
                graph.add_edge(from, to);
            }
        }
    }

    graph
}

fn solve_part2(lines: Lines) -> i64 {
    let graph: DirectedGraph = parse_lines(lines);
    count_possible_paths_with_dac_and_fft(&graph)
}

// For proper memoization we now also need to remember if we have visited the dac and fft nodes.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct MemoizationState {
    pub node_index: usize,
    pub has_seen_dac: bool,
    pub has_seen_fft: bool,
}

impl MemoizationState {
    pub fn new(node_index: usize, has_seen_dac: bool, has_seen_fft: bool) -> Self {
        MemoizationState {
            node_index,
            has_seen_dac,
            has_seen_fft,
        }
    }
}

pub fn count_possible_paths_with_dac_and_fft(graph: &DirectedGraph) -> i64 {
    let start_node_index = graph.get_node_index("svr");
    let end_node_index = graph.get_node_index("out");
    let mut visited: HashMap<MemoizationState, i64> = HashMap::new();

    count_possible_paths_from(
        graph,
        start_node_index,
        end_node_index,
        &mut visited,
        false,
        false,
    )
}

fn count_possible_paths_from(
    graph: &DirectedGraph,
    current_node_index: usize,
    end_node_index: usize,
    visited: &mut HashMap<MemoizationState, i64>,
    has_seen_dac: bool,
    has_seen_fft: bool,
) -> i64 {
    if current_node_index == end_node_index {
        return if has_seen_dac && has_seen_fft { 1 } else { 0 };
    }

    let state = MemoizationState::new(current_node_index, has_seen_dac, has_seen_fft);
    if visited.contains_key(&state) {
        return visited[&state];
    }

    let mut nr_possible_paths: i64 = 0;

    let node = graph.get_node_by_index(current_node_index);
    let has_seen_dac_now = if node.name == "dac" {
        true
    } else {
        has_seen_dac
    };
    let has_seen_fft_now = if node.name == "fft" {
        true
    } else {
        has_seen_fft
    };
    for new_node_index in node.edges.iter() {
        nr_possible_paths += count_possible_paths_from(
            graph,
            *new_node_index,
            end_node_index,
            visited,
            has_seen_dac_now,
            has_seen_fft_now,
        );
    }

    let state = MemoizationState::new(current_node_index, has_seen_dac, has_seen_fft);
    visited.insert(state, nr_possible_paths);

    nr_possible_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        // Arrange
        let input: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;
        let expected: i64 = 5;

        // Act
        let actual: i64 = solve_part1(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part2() {
        // Arrange
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;
        let expected: i64 = 2;

        // Act
        let actual: i64 = solve_part2(input.lines());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_lines() {
        // Arrange
        let input = "ner: ihv mpi bma lls";

        // Act
        let graph = parse_lines(input.lines());

        // Assert
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.get_node("ner").unwrap().edges.len(), 4);
        assert!(graph.has_edge("ner", "ihv"));
        assert!(graph.has_edge("ner", "mpi"));
        assert!(graph.has_edge("ner", "bma"));
        assert!(graph.has_edge("ner", "lls"));
    }
}
