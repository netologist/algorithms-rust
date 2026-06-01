/// Single-decree Paxos consensus.
///
/// Roles:
/// - Proposer: initiates the protocol
/// - Acceptor: votes on proposals  
/// - Learner: learns the decided value (combined with proposer in this demo)

#[derive(Debug, Clone, PartialEq)]
pub enum PaxosError {
    NoQuorum,
    Superseded { higher_ballot: u64 },
}

#[derive(Debug, Clone)]
pub struct PrepareResponse {
    pub promised: bool,
    pub accepted_n: Option<u64>,
    pub accepted_val: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AcceptResponse {
    pub accepted: bool,
}

pub struct Acceptor {
    pub id: u64,
    /// Highest ballot this acceptor has promised not to accept below.
    pub promised_n: Option<u64>,
    /// The ballot of the last accepted proposal.
    pub accepted_n: Option<u64>,
    /// The value of the last accepted proposal.
    pub accepted_val: Option<u64>,
}

impl Acceptor {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            promised_n: None,
            accepted_n: None,
            accepted_val: None,
        }
    }

    /// Phase 1b: Promise not to accept any proposal with ballot < n.
    /// Returns (promised, accepted_n, accepted_val).
    pub fn prepare(&mut self, n: u64) -> PrepareResponse {
        if self.promised_n.is_none_or(|p| n > p) {
            self.promised_n = Some(n);
            PrepareResponse {
                promised: true,
                accepted_n: self.accepted_n,
                accepted_val: self.accepted_val,
            }
        } else {
            PrepareResponse {
                promised: false,
                accepted_n: None,
                accepted_val: None,
            }
        }
    }

    /// Phase 2b: Accept the proposal if the ballot is still the highest promised.
    pub fn accept(&mut self, n: u64, val: u64) -> AcceptResponse {
        if self.promised_n.is_none_or(|p| n >= p) {
            self.promised_n = Some(n);
            self.accepted_n = Some(n);
            self.accepted_val = Some(val);
            AcceptResponse { accepted: true }
        } else {
            AcceptResponse { accepted: false }
        }
    }
}

pub struct PaxosCluster {
    acceptors: Vec<Acceptor>,
    next_ballot: u64,
}

impl PaxosCluster {
    pub fn new(size: usize) -> Self {
        Self {
            acceptors: (1..=size as u64).map(Acceptor::new).collect(),
            next_ballot: 0,
        }
    }

    /// Run a full Paxos round to agree on `value`.
    /// Returns the decided value (may differ from `value` if a previous
    /// accepted value takes precedence).
    pub fn propose(&mut self, value: u64) -> Result<u64, PaxosError> {
        self.next_ballot += 1;
        let n = self.next_ballot;
        let quorum = self.acceptors.len() / 2 + 1;

        // --- Phase 1: Prepare ---
        println!("  Phase 1 — Prepare(n={})", n);
        let mut promises = vec![];
        for a in &mut self.acceptors {
            let resp = a.prepare(n);
            println!(
                "    Acceptor-{}: {}",
                a.id,
                if resp.promised {
                    "Promise ✓"
                } else {
                    "Reject ✗"
                }
            );
            if resp.promised {
                promises.push(resp);
            }
        }

        if promises.len() < quorum {
            return Err(PaxosError::NoQuorum);
        }

        // If any acceptor already accepted a value, use the one with highest ballot
        let proposed = promises
            .iter()
            .filter_map(|p| p.accepted_n.zip(p.accepted_val))
            .max_by_key(|(n, _)| *n)
            .map(|(_, v)| v)
            .unwrap_or(value);

        // --- Phase 2: Accept ---
        println!("  Phase 2 — Accept(n={}, val={})", n, proposed);
        let mut accepts = 0;
        for a in &mut self.acceptors {
            let resp = a.accept(n, proposed);
            println!(
                "    Acceptor-{}: {}",
                a.id,
                if resp.accepted {
                    "Accepted ✓"
                } else {
                    "Rejected ✗"
                }
            );
            if resp.accepted {
                accepts += 1;
            }
        }

        if accepts < quorum {
            return Err(PaxosError::NoQuorum);
        }

        println!("  Learned value: {}", proposed);
        Ok(proposed)
    }
}
