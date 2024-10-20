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

    pub fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (node, neighbors) in &self.adjacency_table {
            for (neighbor, weight) in neighbors {
                edges.push((node, neighbor, *weight));
            }
        }
        edges
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
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
        ];

        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }
}
