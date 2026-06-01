use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct QuorumConfig {
    pub w: usize, // write quorum
    pub r: usize, // read quorum
}

type NodeStore = Arc<Mutex<HashMap<String, (String, u64)>>>; // key → (value, version)

pub struct QuorumCluster {
    nodes: Vec<NodeStore>,
    config: QuorumConfig,
}

impl QuorumCluster {
    pub fn new(n: usize, config: QuorumConfig) -> Self {
        let nodes = (0..n)
            .map(|_| Arc::new(Mutex::new(HashMap::new())))
            .collect();
        Self { nodes, config }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Write to W random nodes with monotonically increasing version.
    pub fn write(&self, key: &str, value: &str) {
        let current_version = self.latest_version(key);
        let new_version = current_version + 1;
        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..self.nodes.len()).collect();
        indices.shuffle(&mut rng);
        for &i in indices.iter().take(self.config.w) {
            self.nodes[i]
                .lock()
                .unwrap()
                .insert(key.into(), (value.into(), new_version));
        }
    }

    /// Write directly to a specific node (for stale-read demo).
    pub fn write_to_node(&self, idx: usize, key: &str, value: &str, version: u64) {
        self.nodes[idx]
            .lock()
            .unwrap()
            .insert(key.into(), (value.into(), version));
    }

    /// Read from R random nodes; return the value with the highest version.
    pub fn read(&self, key: &str) -> Option<String> {
        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..self.nodes.len()).collect();
        indices.shuffle(&mut rng);
        let responses: Vec<(String, u64)> = indices
            .iter()
            .take(self.config.r)
            .filter_map(|&i| self.nodes[i].lock().unwrap().get(key).cloned())
            .collect();
        responses
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(val, _)| val)
    }

    /// Read from a specific node (for testing stale reads).
    pub fn read_from_node(&self, idx: usize, key: &str) -> Option<String> {
        self.nodes[idx]
            .lock()
            .unwrap()
            .get(key)
            .map(|(v, _)| v.clone())
    }

    fn latest_version(&self, key: &str) -> u64 {
        self.nodes
            .iter()
            .filter_map(|n| n.lock().unwrap().get(key).map(|(_, v)| *v))
            .max()
            .unwrap_or(0)
    }
}
