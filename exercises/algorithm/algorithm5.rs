use std::collections::{HashMap, HashSet};

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl UndirectedGraph {
    pub fn new() -> Self {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    pub fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    pub fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    pub fn add_node(&mut self, node: &str) -> bool {
        if !self.adjacency_table.contains_key(node) {
            self.adjacency_table.insert(node.to_string(), Vec::new());
            true
        } else {
            false
        }
    }

    pub fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;
        self.add_node(node1);
        self.add_node(node2);

        self.adjacency_table
            .get_mut(node1)
            .unwrap()
            .push((node2.to_string(), weight));

        self.adjacency_table
            .get_mut(node2)
            .unwrap()
            .push((node1.to_string(), weight));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            ("a", "b", 5),
            ("b", "c", 10),
            ("c", "a", 7),
        ];

        for &(n1, n2, w) in &expected_edges {
            assert!(graph
                .adjacency_table()
                .get(n1)
                .unwrap()
                .iter()
                .any(|(node, weight)| node == n2 && *weight == w));
        }
    }
}
