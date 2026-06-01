use std::collections::HashMap;
use std::time::{Duration, Instant};

pub type NodeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Up,
    Down,
}

pub struct HeartbeatMonitor {
    timeout: Duration,
    last_beat: HashMap<NodeId, Instant>,
}

impl HeartbeatMonitor {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            last_beat: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: NodeId) {
        self.last_beat.insert(id, Instant::now());
    }

    /// Record a heartbeat from `id`.
    pub fn beat(&mut self, id: NodeId) {
        self.last_beat.insert(id, Instant::now());
    }

    /// Return Up if last beat was within timeout, Down otherwise.
    pub fn status(&self, id: NodeId) -> NodeStatus {
        match self.last_beat.get(&id) {
            Some(t) if t.elapsed() <= self.timeout => NodeStatus::Up,
            _ => NodeStatus::Down,
        }
    }

    pub fn all_statuses(&self) -> HashMap<NodeId, NodeStatus> {
        self.last_beat
            .keys()
            .map(|&id| (id, self.status(id)))
            .collect()
    }
}
