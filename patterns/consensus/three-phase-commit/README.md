# Three-Phase Commit (3PC)

## What Is This Pattern?
**3PC** adds a **PreCommit** phase between the vote and the final commit.
This makes the protocol non-blocking: if the coordinator crashes after all
participants have acknowledged PreCommit, participants can safely commit
independently (they know everyone agreed). This fixes 2PC's coordinator
failure problem — but at the cost of being vulnerable to network partitions.

## When To Use It
- Environments with reliable networks where coordinator failures are the primary concern
- Academic/theoretical context for understanding the limits of distributed commitment
- As a stepping stone toward understanding Paxos/Raft

## How It Works

```
Phase 1 — CanCommit
  Coordinator → "Can you commit?" → all participants vote YES/NO

Phase 2 — PreCommit (only if all YES)
  Coordinator → PreCommit → participants ACK
  (participants now KNOW everyone agreed)

Phase 3 — DoCommit
  Coordinator → DoCommit → participants commit

Coordinator crash after Phase 2:
  Participants already know everyone said YES → can commit independently ✓

Coordinator crash after Phase 1 (before PreCommit):
  Participants don't know if others voted YES → safe to abort ✓
```

The key insight: PreCommit is a "no turning back" signal. Once all participants
ACK it, they can always safely commit even without the coordinator.

## Key Rust Concepts Used
- **`coordinator_fails` flag**: simulates crash mid-protocol for demo/tests
- **`PartialEq` on `Decision`**: clean assertions in tests
- **Deterministic simulation**: no threads needed — protocol is sequential

## Run
```bash
cargo run -p three-phase-commit
```

## Test
```bash
cargo test -p three-phase-commit
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Non-blocking on coordinator failure | Not resilient to network partitions |
| Participants can self-resolve after PreCommit | More round-trips than 2PC |
| Theoretical improvement over 2PC | Rarely used in production (Paxos/Raft preferred) |

## Real-World Usage
- Mostly theoretical — production systems use Paxos or Raft instead
- Understanding 3PC explains why Paxos adds the "accept" phase

## Further Reading
- [Skeen: Nonblocking Commit Protocols (1981)](https://dl.acm.org/doi/10.1145/582318.582339)
- [Designing Data-Intensive Applications — Chapter 9](https://dataintensive.net/)
