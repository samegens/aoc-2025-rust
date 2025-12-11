use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct GraphNode {
    pub name: String,
    pub edges: Vec<usize>,
}

impl GraphNode {
    pub fn new(name: &str) -> Self {
        GraphNode {
            name: name.to_string(),
            edges: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct DirectedGraph {
    nodes: Vec<GraphNode>,
    node_map: HashMap<String, usize>,
}

impl DirectedGraph {
    pub fn new() -> Self {
        DirectedGraph {
            nodes: vec![],
            node_map: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.add_node_if_unknown(from);
        self.add_node_if_unknown(to);

        let from_node_index = self.node_map[from];
        let to_node_index = self.node_map[to];

        self.nodes[from_node_index].edges.push(to_node_index);
    }

    pub fn get_node(&self, name: &str) -> Option<&GraphNode> {
        self.node_map.get(name).map(|&idx| &self.nodes[idx])
    }

    pub fn get_node_by_index(&self, index: usize) -> &GraphNode {
        &self.nodes[index]
    }

    pub fn get_neighbors(&self, name: &str) -> Option<Vec<&str>> {
        self.node_map.get(name).map(|&idx| {
            self.nodes[idx]
                .edges
                .iter()
                .map(|&edge_idx| self.nodes[edge_idx].name.as_str())
                .collect()
        })
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        if let (Some(&from_idx), Some(&to_idx)) = (self.node_map.get(from), self.node_map.get(to)) {
            self.nodes[from_idx].edges.contains(&to_idx)
        } else {
            false
        }
    }

    pub fn count_possible_paths(&self, start_node: &str, end_node: &str) -> i64 {
        let start_node_index = self.get_node_index(start_node);
        let end_node_index = self.get_node_index(end_node);
        let mut visited: HashMap<usize, i64> = HashMap::new();

        self.count_possible_paths_from(start_node_index, end_node_index, &mut visited)
    }

    fn add_node_if_unknown(&mut self, name: &str) {
        if !self.node_map.contains_key(name) {
            self.nodes.push(GraphNode::new(name));
            self.node_map.insert(name.to_string(), self.nodes.len() - 1);
        }
    }

    pub fn get_node_index(&self, name: &str) -> usize {
        *self.node_map.get(name).unwrap()
    }

    fn count_possible_paths_from(
        &self,
        current_node_index: usize,
        end_node_index: usize,
        visited: &mut HashMap<usize, i64>,
    ) -> i64 {
        if current_node_index == end_node_index {
            return 1;
        }

        if visited.contains_key(&current_node_index) {
            return visited[&current_node_index];
        }

        let mut nr_possible_paths: i64 = 0;

        let node = &self.nodes[current_node_index];
        for new_node_index in node.edges.iter() {
            nr_possible_paths +=
                self.count_possible_paths_from(*new_node_index, end_node_index, visited);
        }

        visited.insert(current_node_index, nr_possible_paths);

        nr_possible_paths
    }
}

impl Default for DirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph_is_empty() {
        // Act
        let sut = DirectedGraph::new();

        // Assert
        assert_eq!(sut.node_count(), 0);
    }

    #[test]
    fn test_add_single_edge() {
        // Arrange
        let mut sut = DirectedGraph::new();

        // Act
        sut.add_edge("A", "B");

        // Assert
        assert_eq!(sut.node_count(), 2);
        assert!(sut.has_edge("A", "B"));
        assert!(!sut.has_edge("B", "A"));
    }

    #[test]
    fn test_add_multiple_edges_from_same_node() {
        // Arrange
        let mut sut = DirectedGraph::new();

        // Act
        sut.add_edge("A", "B");
        sut.add_edge("A", "C");
        sut.add_edge("A", "D");

        // Assert
        let neighbors = sut.get_neighbors("A").unwrap();
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&"B"));
        assert!(neighbors.contains(&"C"));
        assert!(neighbors.contains(&"D"));
    }

    #[test]
    fn test_get_node_returns_correct_node() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");

        // Act
        let node = sut.get_node("A");

        // Assert
        assert!(node.is_some());
        assert_eq!(node.unwrap().name, "A");
    }

    #[test]
    fn test_get_node_returns_none_for_nonexistent() {
        // Arrange
        let sut = DirectedGraph::new();

        // Act
        let node = sut.get_node("Z");

        // Assert
        assert!(node.is_none());
    }

    #[test]
    fn test_get_neighbors_returns_empty_for_leaf_node() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");

        // Act
        let neighbors = sut.get_neighbors("B").unwrap();

        // Assert
        assert_eq!(neighbors.len(), 0);
    }

    #[test]
    fn test_get_neighbors_returns_none_for_nonexistent() {
        // Arrange
        let sut = DirectedGraph::new();

        // Act
        let neighbors = sut.get_neighbors("Z");

        // Assert
        assert!(neighbors.is_none());
    }

    #[test]
    fn test_duplicate_edges_not_prevented() {
        // Arrange
        let mut sut = DirectedGraph::new();

        // Act
        sut.add_edge("A", "B");
        sut.add_edge("A", "B");

        // Assert
        let neighbors = sut.get_neighbors("A").unwrap();
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn test_complex_graph() {
        // Arrange
        let mut sut = DirectedGraph::new();

        // Act
        sut.add_edge("A", "B");
        sut.add_edge("B", "C");
        sut.add_edge("C", "D");
        sut.add_edge("D", "A");

        // Assert
        assert_eq!(sut.node_count(), 4);
        assert!(sut.has_edge("A", "B"));
        assert!(sut.has_edge("B", "C"));
        assert!(sut.has_edge("C", "D"));
        assert!(sut.has_edge("D", "A"));
        assert!(!sut.has_edge("A", "C"));
    }

    #[test]
    fn test_self_loop() {
        // Arrange
        let mut sut = DirectedGraph::new();

        // Act
        sut.add_edge("A", "A");

        // Assert
        assert_eq!(sut.node_count(), 1);
        assert!(sut.has_edge("A", "A"));
    }

    #[test]
    fn test_default_creates_empty_graph() {
        // Act
        let sut = DirectedGraph::default();

        // Assert
        assert_eq!(sut.node_count(), 0);
    }

    #[test]
    fn test_count_possible_paths_single_path() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");
        sut.add_edge("B", "C");
        sut.add_edge("C", "D");

        // Act
        let actual = sut.count_possible_paths("A", "D");

        // Assert
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_count_possible_paths_two_paths() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");
        sut.add_edge("A", "C");
        sut.add_edge("B", "D");
        sut.add_edge("C", "D");

        // Act
        let actual = sut.count_possible_paths("A", "D");

        // Assert
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_count_possible_paths_multiple_paths() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");
        sut.add_edge("A", "C");
        sut.add_edge("B", "D");
        sut.add_edge("C", "D");
        sut.add_edge("B", "C");

        // Act
        let actual = sut.count_possible_paths("A", "D");

        // Assert
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_count_possible_paths_no_path() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");
        sut.add_edge("C", "D");

        // Act
        let actual = sut.count_possible_paths("A", "D");

        // Assert
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_count_possible_paths_same_start_and_end() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");

        // Act
        let actual = sut.count_possible_paths("A", "A");

        // Assert
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_count_possible_paths_complex_graph() {
        // Arrange
        let mut sut = DirectedGraph::new();
        sut.add_edge("A", "B");
        sut.add_edge("A", "C");
        sut.add_edge("B", "C");
        sut.add_edge("B", "D");
        sut.add_edge("C", "D");
        sut.add_edge("C", "E");
        sut.add_edge("D", "E");

        // Act
        let actual = sut.count_possible_paths("A", "E");

        // Assert
        assert_eq!(actual, 5);
    }
}
