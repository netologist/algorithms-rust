use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub type NodeId = u64;

#[derive(Clone)]
pub struct GossipNode {
    pub id: NodeId,
    pub store: HashMap<String, String>,
    pub alive: bool,
}

impl GossipNode {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            store: HashMap::new(),
            alive: true,
        }
    }
}

pub struct GossipCluster {
    pub nodes: Vec<Arc<Mutex<GossipNode>>>,
    fanout: usize,
}

impl GossipCluster {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: (1..=n as u64)
                .map(|id| Arc::new(Mutex::new(GossipNode::new(id))))
                .collect(),
            fanout: 3,
        }
    }

    /// Seed a node with a key/value rumour.
    pub fn infect(&self, node_idx: usize, key: &str, value: &str) {
        self.nodes[node_idx]
            .lock()
            .unwrap()
            .store
            .insert(key.into(), value.into());
    }

    pub fn kill_node(&self, node_idx: usize) {
        self.nodes[node_idx].lock().unwrap().alive = false;
    }

    /// Fraction of alive nodes that have `key`.
    pub fn coverage(&self, key: &str) -> f64 {
        let alive: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| n.lock().unwrap().alive)
            .collect();
        if alive.is_empty() {
            return 0.0;
        }
        let infected = alive
            .iter()
            .filter(|n| n.lock().unwrap().store.contains_key(key))
            .count();
        infected as f64 / alive.len() as f64
    }

    /// Spawn a background gossip thread for each node.
    pub fn start(&self) {
        for i in 0..self.nodes.len() {
            let nodes = self.nodes.clone();
            let fanout = self.fanout;
            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for _ in 0..20 {
                    thread::sleep(Duration::from_millis(25));
                    let alive = nodes[i].lock().unwrap().alive;
                    if !alive {
                        break;
                    }

                    let store_snapshot = nodes[i].lock().unwrap().store.clone();
                    if store_snapshot.is_empty() {
                        continue;
                    }

                    let mut peers: Vec<usize> = (0..nodes.len()).filter(|&j| j != i).collect();
                    peers.shuffle(&mut rng);

                    for &peer_idx in peers.iter().take(fanout) {
                        let peer_alive = nodes[peer_idx].lock().unwrap().alive;
                        if peer_alive {
                            let mut peer = nodes[peer_idx].lock().unwrap();
                            for (k, v) in &store_snapshot {
                                peer.store.entry(k.clone()).or_insert_with(|| v.clone());
                            }
                        }
                    }
                }
            });
        }
    }
}
