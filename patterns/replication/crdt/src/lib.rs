use std::collections::HashMap;

pub type NodeId = u64;

/// G-Counter: grow-only counter. Each node has its own slot.
/// Value = sum of all slots. Merge = max per slot.
#[derive(Debug, Clone)]
pub struct GCounter {
    node_id: NodeId,
    counts: HashMap<NodeId, u64>,
}

impl GCounter {
    pub fn new(node_id: NodeId) -> Self {
        let mut counts = HashMap::new();
        counts.insert(node_id, 0);
        Self { node_id, counts }
    }

    pub fn increment(&mut self, by: u64) {
        *self.counts.entry(self.node_id).or_insert(0) += by;
    }

    pub fn value(&self) -> u64 {
        self.counts.values().sum()
    }

    /// Merge: take the maximum value per node slot.
    pub fn merge(&mut self, other: &GCounter) {
        for (&id, &count) in &other.counts {
            let entry = self.counts.entry(id).or_insert(0);
            *entry = (*entry).max(count);
        }
    }
}

/// LWW-Register: Last-Write-Wins register.
/// Keeps the value with the highest timestamp.
#[derive(Debug, Clone)]
pub struct LwwRegister {
    pub node_id: NodeId,
    pub value: String,
    pub timestamp: u64,
}

impl LwwRegister {
    pub fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
            value: String::new(),
            timestamp: 0,
        }
    }

    pub fn set(&mut self, value: &str, timestamp: u64) {
        if timestamp > self.timestamp {
            self.value = value.into();
            self.timestamp = timestamp;
        }
    }

    pub fn get(&self) -> &str {
        &self.value
    }

    /// Merge: keep the entry with the higher timestamp.
    pub fn merge(&mut self, other: &LwwRegister) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}
