use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

/// Consistent Hash Ring with virtual nodes.
///
/// Each physical node is mapped to `virtual_nodes` positions on the ring.
/// `get_node(key)` returns the first node clockwise from the key's hash position.
pub struct HashRing {
    ring: BTreeMap<u64, String>, // hash position → physical node name
    virtual_nodes: usize,
}

impl HashRing {
    pub fn new(virtual_nodes: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            virtual_nodes,
        }
    }

    pub fn add_node(&mut self, name: &str) {
        for i in 0..self.virtual_nodes {
            let key = format!("{}#{}", name, i);
            self.ring.insert(hash_str(&key), name.into());
        }
    }

    pub fn remove_node(&mut self, name: &str) {
        for i in 0..self.virtual_nodes {
            let key = format!("{}#{}", name, i);
            self.ring.remove(&hash_str(&key));
        }
    }

    /// Return the node responsible for `key` (first clockwise on the ring).
    pub fn get_node(&self, key: &str) -> &str {
        if self.ring.is_empty() {
            return "";
        }
        let h = hash_str(key);
        // Find first node clockwise (≥ h); wrap around if needed.
        self.ring
            .range(h..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, name)| name.as_str())
            .unwrap_or("")
    }

    pub fn node_count(&self) -> usize {
        self.ring
            .values()
            .collect::<std::collections::HashSet<_>>()
            .len()
    }
}

fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
