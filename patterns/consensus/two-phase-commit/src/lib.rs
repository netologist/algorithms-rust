/// Two-Phase Commit (2PC)
///
/// Phase 1 — Prepare: coordinator asks all participants to vote Yes/No.
/// Phase 2 — Commit/Abort: if all vote Yes → commit; any No → abort.

#[derive(Debug, Clone, PartialEq)]
pub enum ParticipantVote {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Commit,
    Abort,
}

pub struct Participant {
    pub id: u64,
    pub vote: ParticipantVote,
}

impl Participant {
    pub fn new(id: u64, vote: ParticipantVote) -> Self {
        Self { id, vote }
    }
}

pub struct Coordinator {
    participants: Vec<Participant>,
}

impl Coordinator {
    pub fn new(participants: Vec<Participant>) -> Self {
        Self { participants }
    }

    /// Run Phase 1 (Prepare) and Phase 2 (Commit/Abort).
    pub fn run(&self) -> Decision {
        // --- Phase 1: Prepare ---
        println!("  [Phase 1] Coordinator → Prepare");
        let mut all_yes = true;
        for p in &self.participants {
            let vote_str = match p.vote {
                ParticipantVote::Yes => "YES ✓",
                ParticipantVote::No => "NO  ✗",
            };
            println!("    Participant-{}: {}", p.id, vote_str);
            if p.vote == ParticipantVote::No {
                all_yes = false;
            }
        }

        // --- Phase 2: Commit or Abort ---
        let decision = if all_yes {
            Decision::Commit
        } else {
            Decision::Abort
        };
        println!(
            "  [Phase 2] Coordinator → {:?} (broadcast to all participants)",
            decision
        );
        for p in &self.participants {
            println!("    Participant-{}: ACK {:?}", p.id, decision);
        }
        decision
    }
}
