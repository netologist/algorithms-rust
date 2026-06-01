use std::sync::{Arc, Mutex};
use std::time::Duration;

pub type NodeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub status: NodeStatus,
    pub leader: Option<NodeId>,
}

impl Node {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            status: NodeStatus::Alive,
            leader: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.status == NodeStatus::Alive
    }
}

pub struct BullyCluster {
    pub nodes: Vec<Arc<Mutex<Node>>>,
}

impl BullyCluster {
    /// Create a cluster of `n` nodes with IDs 1..=n and run initial election.
    pub fn new(n: usize) -> Self {
        let nodes: Vec<_> = (1..=n as u64)
            .map(|id| Arc::new(Mutex::new(Node::new(id))))
            .collect();
        let cluster = Self { nodes };
        cluster.run_election();
        cluster
    }

    /// Return the current leader (must be the same for all alive nodes).
    pub fn current_leader(&self) -> Option<NodeId> {
        self.nodes
            .iter()
            .filter(|n| n.lock().unwrap().is_alive())
            .find_map(|n| n.lock().unwrap().leader)
    }

    /// Kill a node (mark it dead and clear its leadership).
    pub fn kill_node(&mut self, id: NodeId) {
        for n in &self.nodes {
            let mut n = n.lock().unwrap();
            if n.id == id {
                n.status = NodeStatus::Dead;
            }
            if n.leader == Some(id) {
                n.leader = None;
            }
        }
        println!("  Node-{} killed.", id);
        std::thread::sleep(Duration::from_millis(10));
        self.run_election();
    }

    /// Run the bully algorithm.
    ///
    /// Any alive node with no known leader can start an election.
    /// The node with the highest alive ID wins.
    pub fn run_election(&self) {
        let max_alive_id = self
            .nodes
            .iter()
            .filter(|n| n.lock().unwrap().is_alive())
            .map(|n| n.lock().unwrap().id)
            .max();

        if let Some(winner) = max_alive_id {
            println!("  Election → Node-{} wins (highest alive ID)", winner);
            for n in &self.nodes {
                let mut n = n.lock().unwrap();
                if n.is_alive() {
                    n.leader = Some(winner);
                }
            }
        }
    }
}
