# Consensus Patterns

Algorithms that allow a group of distributed nodes to agree on a single value
or sequence of operations, even when some nodes fail or the network partitions.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Leader Election](leader-election/) | `leader-election` | Bully algorithm — highest-ID node becomes leader; detects failure and re-elects |
| [Two-Phase Commit](two-phase-commit/) | `two-phase-commit` | 2PC — coordinator proposes, participants vote, coordinator commits or aborts |
| [Three-Phase Commit](three-phase-commit/) | `three-phase-commit` | 3PC — adds pre-commit phase to avoid blocking on coordinator failure |
| [Paxos](paxos/) | `paxos` | Classic Paxos — proposers, acceptors, learners; majority quorum for agreement |
| [Raft](raft/) | `raft` | Leader-based consensus with randomized election timeouts; easier to understand than Paxos |

## Key Rust Concepts

- Async state machines with `tokio` tasks and channels
- Enum-based protocol messages dispatched via pattern matching
- `Arc<Mutex<T>>` for shared node state across concurrent tasks
- Timer-driven elections and heartbeats with `tokio::time`
