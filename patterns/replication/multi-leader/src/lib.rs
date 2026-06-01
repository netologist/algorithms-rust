use std::collections::HashMap;

pub type NodeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum ConflictResolution {
    LastWriteWins,
}

#[derive(Debug, Clone)]
pub struct VersionedValue {
    pub value: String,
    pub timestamp: u64,
    pub origin: NodeId,
}

pub struct MultiLeaderCluster {
    leaders: Vec<HashMap<String, VersionedValue>>,
    resolution: ConflictResolution,
}

impl MultiLeaderCluster {
    pub fn new(n: usize, resolution: ConflictResolution) -> Self {
        Self {
            leaders: vec![HashMap::new(); n],
            resolution,
        }
    }

    /// Write a key on a specific leader.
    pub fn write(&mut self, leader_idx: usize, key: &str, value: &str, ts: u64) {
        self.leaders[leader_idx].insert(
            key.into(),
            VersionedValue {
                value: value.into(),
                timestamp: ts,
                origin: leader_idx as NodeId,
            },
        );
    }

    /// Sync all leaders: merge using the configured conflict resolution strategy.
    pub fn sync(&mut self) {
        let n = self.leaders.len();
        // Collect all keys
        let keys: Vec<String> = self
            .leaders
            .iter()
            .flat_map(|l| l.keys().cloned())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for key in keys {
            let winner = self
                .leaders
                .iter()
                .filter_map(|l| l.get(&key))
                .max_by_key(|v| v.timestamp)
                .cloned();

            if let Some(w) = winner {
                for i in 0..n {
                    self.leaders[i].insert(key.clone(), w.clone());
                }
            }
        }
    }

    pub fn read(&self, key: &str) -> Option<&str> {
        // Read from the first leader (after sync all should agree)
        self.leaders[0].get(key).map(|v| v.value.as_str())
    }

    pub fn read_from(&self, leader_idx: usize, key: &str) -> Option<&str> {
        self.leaders[leader_idx].get(key).map(|v| v.value.as_str())
    }
}
