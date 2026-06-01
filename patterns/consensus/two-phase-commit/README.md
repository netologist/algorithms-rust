# Two-Phase Commit (2PC)

## What Is This Pattern?
**2PC** is a distributed protocol that ensures all participants in a transaction
either commit or abort — atomically. It coordinates a **coordinator** and N
**participants** through two phases: vote collection and the final decision.

## When To Use It
- Distributed database transactions spanning multiple shards
- Distributed sagas where all steps must atomically succeed or roll back
- Any multi-node operation requiring all-or-nothing semantics

## How It Works

```
Coordinator         Participant 1    Participant 2    Participant 3
     │                    │               │               │
     ├─── Prepare ───────►│◄─────────────►│◄─────────────►│
     │◄── Vote(YES) ──────┤               │               │
     │◄── Vote(YES) ──────────────────────┤               │
     │◄── Vote(NO)  ──────────────────────────────────────┤
     │                                                     │
     ├─── Abort ──────────►│◄─────────────►│◄─────────────►│
     │◄── ACK ─────────────┤               │               │
```

**Phase 1 — Prepare:**
1. Coordinator sends `Prepare` to all participants.
2. Each participant votes `Yes` (can commit) or `No` (must abort).

**Phase 2 — Commit/Abort:**
- All `Yes` → Coordinator sends `Commit` to all.
- Any `No` → Coordinator sends `Abort` to all.

**Problem:** If the coordinator crashes after Phase 1, participants are stuck
in an uncertain state. This is 2PC's fundamental limitation. 3PC addresses it.

## Key Rust Concepts Used
- **Enum variants with data**: `ParticipantVote::Yes/No`, `Decision::Commit/Abort`
- **`PartialEq` derive**: enables `assert_eq!` in tests
- **Iterative scan**: finding any `No` vote terminates the transaction

## Run
```bash
cargo run -p two-phase-commit
```

## Test
```bash
cargo test -p two-phase-commit
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Strong atomicity guarantee | Blocks indefinitely if coordinator crashes |
| Simple 2-round protocol | Participants lock resources until Phase 2 |
| Widely understood | Not partition-tolerant (CP in CAP terms) |

## Real-World Usage
- **PostgreSQL** `PREPARE TRANSACTION` / `COMMIT PREPARED`
- **XA Transactions** (Java EE, JDBC)
- **Distributed sagas** (simplified variant)

## Further Reading
- [Gray: Notes on Database Operating Systems (1978)](https://jimgray.azurewebsites.net/papers/dbos.pdf)
- [Designing Data-Intensive Applications — Chapter 9](https://dataintensive.net/)
