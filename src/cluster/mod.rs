pub mod client;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct NodeSelector {
    nodes: Vec<String>, // "host:port" strings
}

impl NodeSelector {
    pub fn new(nodes: Vec<String>) -> Self {
        let mut sorted_nodes = nodes;
        sorted_nodes.sort(); // Ensure deterministic order across all nodes
        Self {
            nodes: sorted_nodes,
        }
    }

    pub fn get_node_for_stream(&self, stream_id: &str) -> String {
        let mut hasher = DefaultHasher::new();
        stream_id.hash(&mut hasher);
        let hash = hasher.finish();

        let idx = (hash as usize) % self.nodes.len();
        self.nodes[idx].clone()
    }

    pub fn get_all_nodes(&self) -> &[String] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let nodes = vec!["127.0.0.1:50051".to_string(), "127.0.0.1:50052".to_string()];
        let selector = NodeSelector::new(nodes);

        // "stream-1" should always hash to the same node
        let node_a = selector.get_node_for_stream("stream-1");
        let node_b = selector.get_node_for_stream("stream-1");

        assert_eq!(node_a, node_b);
    }
}
