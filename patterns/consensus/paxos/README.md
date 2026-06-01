# Paxos

## What Is This Pattern?
**Paxos** is the foundational distributed consensus algorithm. It allows a set of
nodes to agree on a single value even when messages are lost or delayed and some
nodes fail. This implementation covers **Single-Decree Paxos** — consensus on one
value. Multi-Paxos extends this to a log of values.

## When To Use It
- Foundation of most distributed database and coordination systems
- Understanding Raft (which is Paxos restated more clearly)
- Any system needing fault-tolerant agreement among a fixed set of acceptors

## How It Works

```
Proposer          Acceptors (3 of 5)          Learner
    │                   │   │   │                │
    ├─ Prepare(n=1) ───►│   │   │                │
    │◄─ Promise(n=1) ───┤   │   │                │
    │◄─ Promise(n=1) ───────┤   │                │
    │◄─ Promise(n=1) ───────────┤                │
    │                                            │
    ├─ Accept(n=1, v=42) ──►│   │   │            │
    │◄─ Accepted ───────────┤   │   │            │
    │◄─ Accepted ───────────────┤   │            │
    │                                            │
    ├─────────────────────────── Learn(v=42) ───►│
```

**Phase 1 — Prepare:**
1. Proposer picks ballot number `n` (unique, larger than any seen).
2. Sends `Prepare(n)` to all acceptors.
3. Acceptor responds `Promise(n, accepted_n, accepted_val)` if `n > promised_n`.

**Phase 2 — Accept:**
1. If quorum promises received: pick value (highest accepted_val, or own if none).
2. Send `Accept(n, value)` to all.
3. Acceptor accepts if `n >= promised_n`.
4. If quorum accepts → value is decided → notify learners.

**Key invariant:** If any acceptor has already accepted a value, that value must be
chosen in all future rounds (ensuring single-value agreement).

## This Implementation
- `Acceptor` — models a single Paxos acceptor with `prepare/accept` methods
- `PaxosCluster` — orchestrates a round of Paxos across `n` acceptors
- Proposer and learner are combined (single-process simulation)
- Prints each phase message so you can trace the protocol

## Key Rust Concepts Used
- **Method chaining**: `promises.iter().filter_map(...).max_by_key(...)` for phase-2 value selection
- **`Option<T>` for unset state**: `promised_n`, `accepted_n`, `accepted_val`
- **`zip` for paired options**: `accepted_n.zip(accepted_val)`

## Run
```bash
cargo run -p paxos
```

## Test
```bash
cargo test -p paxos
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Theoretically optimal (FLP-minimal) | Hard to understand and implement correctly |
| Handles arbitrary message loss/delay | Two round-trips per value (high latency) |
| Foundation of stronger algorithms | Single-decree only — Multi-Paxos adds complexity |

## Real-World Usage
- **Google Chubby** — Paxos for distributed lock service
- **Apache Zookeeper** — Zab protocol (inspired by Paxos)
- **Raft** — Paxos restated to be understandable

## Further Reading
- [Paxos Made Simple — Leslie Lamport (2001)](https://lamport.azurewebsites.net/pubs/paxos-simple.pdf)
- [Paxos Made Live — Google (2007)](https://research.google/pubs/paxos-made-live-an-engineering-perspective/)
