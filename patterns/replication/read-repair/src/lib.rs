use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct VersionedValue {
    pub value: String,
    pub version: u64,
}

type NodeStore = Arc<Mutex<HashMap<String, VersionedValue>>>;

pub struct ReadRepairCluster {
    nodes: Vec<NodeStore>,
}

impl ReadRepairCluster {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: (0..n)
                .map(|_| Arc::new(Mutex::new(HashMap::new())))
                .collect(),
        }
    }

    /// Write to all nodes.
    pub fn write(&self, key: &str, value: &str) {
        let version = self.latest_version(key) + 1;
        for node in &self.nodes {
            node.lock().unwrap().insert(
                key.into(),
                VersionedValue {
                    value: value.into(),
                    version,
                },
            );
        }
    }

    /// Write directly to a specific node (for injecting stale data).
    pub fn write_stale(&self, node_idx: usize, key: &str, value: &str, version: u64) {
        self.nodes[node_idx].lock().unwrap().insert(
            key.into(),
            VersionedValue {
                value: value.into(),
                version,
            },
        );
    }

    /// Read from all nodes. If any are stale (lower version), repair them in background.
    /// Returns the latest value found.
    pub fn read(&self, key: &str) -> Option<String> {
        let results: Vec<Option<VersionedValue>> = self
            .nodes
            .iter()
            .map(|n| n.lock().unwrap().get(key).cloned())
            .collect();

        let latest = results
            .iter()
            .flatten()
            .max_by_key(|v| v.version)
            .cloned()?;

        // Repair stale nodes in the background
        let stale_indices: Vec<usize> = results
            .iter()
            .enumerate()
            .filter(|(_, r)| r.as_ref().is_none_or(|v| v.version < latest.version))
            .map(|(i, _)| i)
            .collect();

        if !stale_indices.is_empty() {
            let nodes_clone: Vec<NodeStore> = self.nodes.clone();
            let key_owned = key.to_string();
            let latest_clone = latest.clone();
            thread::spawn(move || {
                for i in stale_indices {
                    println!(
                        "  [read-repair] Repairing node-{}: version {} → {}",
                        i, 0, latest_clone.version
                    );
                    nodes_clone[i]
                        .lock()
                        .unwrap()
                        .insert(key_owned.clone(), latest_clone.clone());
                }
            });
        }

        Some(latest.value)
    }

    pub fn read_from_node(&self, idx: usize, key: &str) -> Option<VersionedValue> {
        self.nodes[idx].lock().unwrap().get(key).cloned()
    }

    fn latest_version(&self, key: &str) -> u64 {
        self.nodes
            .iter()
            .filter_map(|n| n.lock().unwrap().get(key).map(|v| v.version))
            .max()
            .unwrap_or(0)
    }
}
