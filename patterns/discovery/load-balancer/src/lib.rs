/// Load Balancer with three algorithms:
/// - RoundRobin: cycle through backends in order
/// - LeastConnections: pick backend with fewest active connections
/// - ConsistentHash: same key always routes to same backend
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq)]
pub enum Algorithm {
    RoundRobin,
    LeastConnections,
    ConsistentHash,
}

#[derive(Debug, Clone)]
pub struct Backend {
    pub id: String,
    pub active_connections: usize,
}

pub struct LoadBalancer {
    backends: Vec<Backend>,
    algorithm: Algorithm,
    rr_index: usize,
}

impl LoadBalancer {
    pub fn new(backends: Vec<String>, algorithm: Algorithm) -> Self {
        Self {
            backends: backends
                .into_iter()
                .map(|id| Backend {
                    id,
                    active_connections: 0,
                })
                .collect(),
            algorithm,
            rr_index: 0,
        }
    }

    /// Select a backend for a request. `key` is used for ConsistentHash only.
    pub fn select(&mut self, key: Option<&str>) -> &str {
        match self.algorithm {
            Algorithm::RoundRobin => {
                let idx = self.rr_index % self.backends.len();
                self.rr_index += 1;
                &self.backends[idx].id
            }
            Algorithm::LeastConnections => {
                let idx = self
                    .backends
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, b)| b.active_connections)
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                &self.backends[idx].id
            }
            Algorithm::ConsistentHash => {
                let k = key.unwrap_or("default");
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                let h = hasher.finish() as usize;
                &self.backends[h % self.backends.len()].id
            }
        }
    }

    pub fn connect(&mut self, backend_id: &str) {
        if let Some(b) = self.backends.iter_mut().find(|b| b.id == backend_id) {
            b.active_connections += 1;
        }
    }

    pub fn disconnect(&mut self, backend_id: &str) {
        if let Some(b) = self.backends.iter_mut().find(|b| b.id == backend_id) {
            b.active_connections = b.active_connections.saturating_sub(1);
        }
    }

    pub fn backends(&self) -> &[Backend] {
        &self.backends
    }
}
