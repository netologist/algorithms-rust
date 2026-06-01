use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Rendezvous (Highest Random Weight) Hashing.
///
/// For a given key, compute hash(key + node) for every node.
/// The node with the highest score wins.
/// When a node is removed, only its keys are remapped.
pub struct RendezvousHasher {
    nodes: Vec<String>,
}

impl RendezvousHasher {
    pub fn new(nodes: Vec<String>) -> Self {
        Self { nodes }
    }

    pub fn add_node(&mut self, name: &str) {
        if !self.nodes.iter().any(|n| n == name) {
            self.nodes.push(name.into());
        }
    }

    pub fn remove_node(&mut self, name: &str) {
        self.nodes.retain(|n| n != name);
    }

    /// Return the node with the highest hash(key + node).
    pub fn get_node(&self, key: &str) -> &str {
        self.nodes
            .iter()
            .max_by_key(|node| self.score(key, node))
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn score(&self, key: &str, node: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", key, node).hash(&mut hasher);
        hasher.finish()
    }
}
