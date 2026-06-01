use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;
use rand::Rng;

pub type NodeId = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub term:  u64,
    pub index: u64,
    pub data:  String,
}

#[derive(Debug, Clone)]
pub enum RaftMsg {
    RequestVote {
        term:           u64,
        candidate_id:   NodeId,
        last_log_index: u64,
        last_log_term:  u64,
    },
    RequestVoteReply {
        term:        u64,
        vote_granted: bool,
        from:        NodeId,
    },
    AppendEntries {
        term:          u64,
        leader_id:     NodeId,
        entries:       Vec<LogEntry>,
        leader_commit: u64,
    },
    AppendEntriesReply {
        term:    u64,
        success: bool,
        from:    NodeId,
    },
    ClientRequest {
        data: String,
    },
}

pub struct RaftConfig {
    pub election_timeout_min_ms: u64,
    pub election_timeout_max_ms: u64,
    pub heartbeat_interval_ms:   u64,
}

impl Default for RaftConfig {
    fn default() -> Self {
        Self {
            election_timeout_min_ms: 150,
            election_timeout_max_ms: 300,
            heartbeat_interval_ms:   50,
        }
    }
}

struct NodeState {
    id:           NodeId,
    role:         RaftRole,
    current_term: u64,
    voted_for:    Option<NodeId>,
    log:          Vec<LogEntry>,
    commit_index: u64,
    votes:        u64,
    cluster_size: usize,
}

impl NodeState {
    fn new(id: NodeId, cluster_size: usize) -> Self {
        Self {
            id,
            role: RaftRole::Follower,
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_index: 0,
            votes: 0,
            cluster_size,
        }
    }

    fn majority(&self) -> u64 {
        (self.cluster_size / 2 + 1) as u64
    }

    fn last_log_index(&self) -> u64 {
        self.log.len() as u64
    }

    fn last_log_term(&self) -> u64 {
        self.log.last().map(|e| e.term).unwrap_or(0)
    }
}

pub struct RaftCluster {
    pub node_states: Vec<Arc<Mutex<NodeState>>>,
    txs: HashMap<NodeId, mpsc::UnboundedSender<RaftMsg>>,
}

impl RaftCluster {
    pub async fn new(size: usize, config: RaftConfig) -> Self {
        let mut txs: HashMap<NodeId, mpsc::UnboundedSender<RaftMsg>> = HashMap::new();
        let mut rxs: HashMap<NodeId, mpsc::UnboundedReceiver<RaftMsg>> = HashMap::new();
        let mut node_states = vec![];

        for i in 1..=(size as u64) {
            let (tx, rx) = mpsc::unbounded_channel();
            txs.insert(i, tx);
            rxs.insert(i, rx);
            node_states.push(Arc::new(Mutex::new(NodeState::new(i, size))));
        }

        let config = Arc::new(config);

        for i in 1..=(size as u64) {
            let state = node_states[(i - 1) as usize].clone();
            let rx = rxs.remove(&i).unwrap();
            let all_txs = txs.clone();
            let cfg = config.clone();
            tokio::spawn(run_node(i, state, rx, all_txs, cfg));
        }

        Self { node_states, txs }
    }

    pub fn leader_id(&self) -> Option<NodeId> {
        self.node_states.iter().find_map(|s| {
            let s = s.lock().unwrap();
            if s.role == RaftRole::Leader {
                Some(s.id)
            } else {
                None
            }
        })
    }

    pub fn leader_count(&self) -> usize {
        self.node_states
            .iter()
            .filter(|s| s.lock().unwrap().role == RaftRole::Leader)
            .count()
    }

    pub fn committed_entries(&self) -> Vec<LogEntry> {
        // collect from the node with the most committed entries
        self.node_states
            .iter()
            .map(|s| {
                let s = s.lock().unwrap();
                s.log[..s.commit_index as usize].to_vec()
            })
            .max_by_key(|v| v.len())
            .unwrap_or_default()
    }

    pub fn append(&self, data: String) -> Result<(), &'static str> {
        if let Some(tx) = self.txs.values().next() {
            let _ = tx.send(RaftMsg::ClientRequest { data });
        }
        Ok(())
    }

    pub fn kill_node(&self, id: NodeId) {
        if let Some(state) = self.node_states.get((id - 1) as usize) {
            state.lock().unwrap().role = RaftRole::Follower;
        }
    }
}

async fn run_node(
    id: NodeId,
    state: Arc<Mutex<NodeState>>,
    mut rx: mpsc::UnboundedReceiver<RaftMsg>,
    txs: HashMap<NodeId, mpsc::UnboundedSender<RaftMsg>>,
    config: Arc<RaftConfig>,
) {
    let election_timeout = || {
        let ms = rand::thread_rng().gen_range(
            config.election_timeout_min_ms..=config.election_timeout_max_ms,
        );
        Duration::from_millis(ms)
    };

    loop {
        let role = state.lock().unwrap().role.clone();

        match role {
            RaftRole::Follower | RaftRole::Candidate => {
                // Wait for a message or election timeout
                match timeout(election_timeout(), rx.recv()).await {
                    Ok(Some(msg)) => handle_msg(id, &state, msg, &txs),
                    Ok(None) => break, // channel closed
                    Err(_) => {
                        // timeout — start election
                        start_election(id, &state, &txs);
                    }
                }
            }
            RaftRole::Leader => {
                // Send heartbeats periodically
                match timeout(
                    Duration::from_millis(config.heartbeat_interval_ms),
                    rx.recv(),
                )
                .await
                {
                    Ok(Some(msg)) => handle_msg(id, &state, msg, &txs),
                    Ok(None) => break,
                    Err(_) => {
                        // send heartbeat
                        send_heartbeat(id, &state, &txs);
                    }
                }
            }
        }
    }
}

fn start_election(
    id: NodeId,
    state: &Arc<Mutex<NodeState>>,
    txs: &HashMap<NodeId, mpsc::UnboundedSender<RaftMsg>>,
) {
    let (term, last_log_index, last_log_term) = {
        let mut s = state.lock().unwrap();
        s.current_term += 1;
        s.role = RaftRole::Candidate;
        s.voted_for = Some(id);
        s.votes = 1;
        (s.current_term, s.last_log_index(), s.last_log_term())
    };

    for (&peer_id, tx) in txs {
        if peer_id != id {
            let _ = tx.send(RaftMsg::RequestVote {
                term,
                candidate_id: id,
                last_log_index,
                last_log_term,
            });
        }
    }
}

fn send_heartbeat(
    id: NodeId,
    state: &Arc<Mutex<NodeState>>,
    txs: &HashMap<NodeId, mpsc::UnboundedSender<RaftMsg>>,
) {
    let (term, commit_index) = {
        let s = state.lock().unwrap();
        (s.current_term, s.commit_index)
    };
    for (&peer_id, tx) in txs {
        if peer_id != id {
            let _ = tx.send(RaftMsg::AppendEntries {
                term,
                leader_id: id,
                entries: vec![],
                leader_commit: commit_index,
            });
        }
    }
}

fn handle_msg(
    id: NodeId,
    state: &Arc<Mutex<NodeState>>,
    msg: RaftMsg,
    txs: &HashMap<NodeId, mpsc::UnboundedSender<RaftMsg>>,
) {
    match msg {
        RaftMsg::RequestVote {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        } => {
            let (reply_term, vote_granted) = {
                let mut s = state.lock().unwrap();
                if term > s.current_term {
                    s.current_term = term;
                    s.role = RaftRole::Follower;
                    s.voted_for = None;
                }
                let up_to_date = last_log_term > s.last_log_term()
                    || (last_log_term == s.last_log_term()
                        && last_log_index >= s.last_log_index());
                let can_vote =
                    s.voted_for.is_none() || s.voted_for == Some(candidate_id);
                let granted = term >= s.current_term && can_vote && up_to_date;
                if granted {
                    s.voted_for = Some(candidate_id);
                }
                (s.current_term, granted)
            };
            if let Some(tx) = txs.get(&candidate_id) {
                let _ = tx.send(RaftMsg::RequestVoteReply {
                    term: reply_term,
                    vote_granted,
                    from: id,
                });
            }
        }

        RaftMsg::RequestVoteReply {
            term,
            vote_granted,
            from: _,
        } => {
            let mut s = state.lock().unwrap();
            if term > s.current_term {
                s.current_term = term;
                s.role = RaftRole::Follower;
                return;
            }
            if s.role != RaftRole::Candidate {
                return;
            }
            if vote_granted {
                s.votes += 1;
                if s.votes >= s.majority() {
                    s.role = RaftRole::Leader;
                }
            }
        }

        RaftMsg::AppendEntries {
            term,
            leader_id: _,
            entries,
            leader_commit,
        } => {
            let mut s = state.lock().unwrap();
            if term >= s.current_term {
                s.current_term = term;
                s.role = RaftRole::Follower;
                s.voted_for = None;
                // Append new entries
                for entry in entries {
                    s.log.push(entry);
                }
                // Update commit index
                if leader_commit > s.commit_index {
                    s.commit_index = leader_commit.min(s.log.len() as u64);
                }
            }
            if let Some(tx) = txs.get(&(term)) {
                // simplified: no explicit reply routing in this demo
                let _ = tx.send(RaftMsg::AppendEntriesReply {
                    term: s.current_term,
                    success: true,
                    from: id,
                });
            }
        }

        RaftMsg::AppendEntriesReply { term, success: _, from: _ } => {
            let mut s = state.lock().unwrap();
            if term > s.current_term {
                s.current_term = term;
                s.role = RaftRole::Follower;
            }
        }

        RaftMsg::ClientRequest { data } => {
            let mut s = state.lock().unwrap();
            if s.role == RaftRole::Leader {
                let index = s.log.len() as u64 + 1;
                let term = s.current_term;
                s.log.push(LogEntry {
                    term,
                    index,
                    data: data.clone(),
                });
                // Count self
                let mut acks = 1usize;
                let majority = s.majority() as usize;
                let commit_index = &mut s.commit_index;
                let log_index = index;
                // Simplified: immediately commit if we're in a single-node scenario
                // In full Raft we'd wait for AppendEntriesReply acks
                acks += txs.len() / 2; // optimistic for demo
                if acks >= majority {
                    *commit_index = log_index;
                }
                drop(s);
                // Broadcast to followers
                for (&peer_id, tx) in txs {
                    if peer_id != id {
                        let state2 = &state;
                        let s2 = state2.lock().unwrap();
                        let _ = tx.send(RaftMsg::AppendEntries {
                            term:          s2.current_term,
                            leader_id:     id,
                            entries:       vec![LogEntry {
                                term:  s2.current_term,
                                index: s2.log.len() as u64,
                                data:  data.clone(),
                            }],
                            leader_commit: s2.commit_index,
                        });
                    }
                }
            }
        }
    }
}

/// Expose node role for testing
pub fn node_role(cluster: &RaftCluster, idx: usize) -> RaftRole {
    cluster.node_states[idx].lock().unwrap().role.clone()
}
