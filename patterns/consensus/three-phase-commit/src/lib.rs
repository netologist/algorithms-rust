/// Three-Phase Commit (3PC)
///
/// Adds a CanCommit phase before PreCommit to handle coordinator crashes.
/// Phase 1 — CanCommit: coordinator asks "can you commit?" — participants vote.
/// Phase 2 — PreCommit: coordinator sends PreCommit — participants ACK.
/// Phase 3 — DoCommit: coordinator sends DoCommit — participants commit.
///
/// Key advantage over 2PC: participants in PreCommit can commit independently
/// if the coordinator crashes (they already agreed via CanCommit).

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Commit,
    Abort,
}

pub struct Coordinator {
    n: usize,
    coordinator_fails: bool, // simulates crash after CanCommit
}

impl Coordinator {
    /// `coordinator_fails`: if true, coordinator crashes after Phase 1
    pub fn new(n: usize, coordinator_fails: bool) -> Self {
        Self {
            n,
            coordinator_fails,
        }
    }

    pub fn run(&self) -> Decision {
        // --- Phase 1: CanCommit ---
        println!("  [Phase 1] Coordinator → CanCommit?");
        for i in 1..=self.n {
            println!("    Participant-{}: YES", i);
        }

        if self.coordinator_fails {
            println!("  *** Coordinator CRASHES after Phase 1 ***");
            println!("  Participants timeout → unanimous abort (safety)");
            return Decision::Abort;
        }

        // --- Phase 2: PreCommit ---
        println!("  [Phase 2] Coordinator → PreCommit");
        for i in 1..=self.n {
            println!("    Participant-{}: ACK PreCommit", i);
        }

        // --- Phase 3: DoCommit ---
        println!("  [Phase 3] Coordinator → DoCommit");
        for i in 1..=self.n {
            println!("    Participant-{}: COMMITTED ✓", i);
        }

        Decision::Commit
    }
}
