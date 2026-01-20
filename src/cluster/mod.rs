pub mod client;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq)] // Added Debug/PartialEq
pub struct ChainOwner {
    pub node_addr: String,
    pub epoch: u64,
}

#[derive(Clone)]
pub struct ClusterTopology {
    nodes: Vec<String>,
    epoch: u64,
}

impl ClusterTopology {
    pub fn new(nodes: Vec<String>, epoch: u64) -> Self {
        let mut sorted_nodes = nodes;
        sorted_nodes.sort();
        Self {
            nodes: sorted_nodes,
            epoch,
        }
    }

    pub fn get_owner(&self, stream_id: &str) -> ChainOwner {
        let mut hasher = DefaultHasher::new();
        stream_id.hash(&mut hasher);
        let hash = hasher.finish();

        let idx = (hash as usize) % self.nodes.len();
        ChainOwner {
            node_addr: self.nodes[idx].clone(),
            epoch: self.epoch,
        }
    }

    pub fn get_all_nodes(&self) -> &[String] {
        &self.nodes
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let nodes = vec!["127.0.0.1:50051".to_string(), "127.0.0.1:50052".to_string()];
        let topology = ClusterTopology::new(nodes, 1);

        // "stream-1" should always hash to the same node
        let owner_a = topology.get_owner("stream-1");
        let owner_b = topology.get_owner("stream-1");

        assert_eq!(owner_a, owner_b);
        assert_eq!(owner_a.epoch, 1);
    }

    #[test]
    fn test_epoch_usage() {
        let nodes = vec!["A".to_string(), "B".to_string()];
        let t1 = ClusterTopology::new(nodes.clone(), 10);

        let o1 = t1.get_owner("stream-x");
        assert_eq!(o1.epoch, 10);

        // New topology with higher epoch
        let t2 = ClusterTopology::new(nodes, 20);
        let o2 = t2.get_owner("stream-x");
        assert_eq!(o2.epoch, 20);
        assert_eq!(o1.node_addr, o2.node_addr); // Owner shouldn't change if nodes same
    }

    #[test]
    fn test_distribution_change() {
        let nodes1 = vec!["A".to_string(), "B".to_string()];
        let t1 = ClusterTopology::new(nodes1, 1);

        let nodes2 = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let t2 = ClusterTopology::new(nodes2, 2);

        // Distribution changes, but epoch tracks it
        let o1 = t1.get_owner("stream-1");
        let o2 = t2.get_owner("stream-1");

        // Usually, hashes differ with node count.
        // We just assert they return valid owners from their respective sets.
        assert!(t1.get_all_nodes().contains(&o1.node_addr));
        assert!(t2.get_all_nodes().contains(&o2.node_addr));
        assert_ne!(o1.epoch, o2.epoch);
    }
}
