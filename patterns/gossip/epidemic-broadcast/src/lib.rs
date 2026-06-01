use rand::seq::SliceRandom;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub type NodeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum InfectionState {
    Susceptible,
    Infected,
}

#[derive(Clone)]
pub struct EpidemicNode {
    pub id: NodeId,
    pub state: InfectionState,
    pub alive: bool,
    pub data: Option<String>,
}

impl EpidemicNode {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            state: InfectionState::Susceptible,
            alive: true,
            data: None,
        }
    }

    pub fn is_infected(&self) -> bool {
        self.state == InfectionState::Infected
    }
}

pub struct EpidemicCluster {
    pub nodes: Vec<Arc<Mutex<EpidemicNode>>>,
    fanout: usize,
}

impl EpidemicCluster {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: (1..=n as u64)
                .map(|id| Arc::new(Mutex::new(EpidemicNode::new(id))))
                .collect(),
            fanout: 3,
        }
    }

    pub fn kill_node(&self, idx: usize) {
        self.nodes[idx].lock().unwrap().alive = false;
    }

    pub fn infected_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.lock().unwrap().is_infected())
            .count()
    }

    pub fn alive_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.lock().unwrap().alive)
            .count()
    }

    /// Broadcast `data` starting from node 0, using SI epidemic spreading.
    pub fn broadcast(&self, data: &str) {
        // Infect node 0
        let mut n0 = self.nodes[0].lock().unwrap();
        n0.state = InfectionState::Infected;
        n0.data = Some(data.into());
        drop(n0);

        let data_owned = data.to_string();
        for i in 0..self.nodes.len() {
            let nodes = self.nodes.clone();
            let d = data_owned.clone();
            let fanout = self.fanout;
            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for _ in 0..20 {
                    thread::sleep(Duration::from_millis(25));
                    let is_infected = {
                        let n = nodes[i].lock().unwrap();
                        n.is_infected() && n.alive
                    };
                    if !is_infected {
                        continue;
                    }

                    let mut peers: Vec<usize> = (0..nodes.len()).filter(|&j| j != i).collect();
                    peers.shuffle(&mut rng);
                    for &peer_idx in peers.iter().take(fanout) {
                        let mut peer = nodes[peer_idx].lock().unwrap();
                        if peer.alive && peer.state == InfectionState::Susceptible {
                            peer.state = InfectionState::Infected;
                            peer.data = Some(d.clone());
                        }
                    }
                }
            });
        }
    }
}
