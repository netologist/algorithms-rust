# Multi-Leader Replication

## What Is This Pattern?
Multiple nodes each accept writes independently (each is a **leader**). Changes
are asynchronously propagated to all other leaders. When two leaders accept
conflicting writes for the same key, a **conflict resolution** strategy
(e.g. Last-Write-Wins) determines the winner.

## When To Use It
- Multi-datacenter deployments (one leader per DC)
- Offline-capable apps (device is a leader; syncs when online)
- High write throughput across geographies

## How It Works

```
DC-A: [Leader-A] ◄─── conflicting writes ───► [Leader-B] :DC-B
         │                                          │
         └──────── sync (LWW resolve) ─────────────┘
```

1. Both leaders accept writes independently.
2. On sync, conflicts detected (same key, different values).
3. LWW: value with highest timestamp wins and overwrites all leaders.

## Key Rust Concepts Used
- **`HashMap<String, VersionedValue>`**: per-leader store with timestamps
- **`HashSet` for dedup**: collect all keys across leaders
- **`max_by_key`**: pick winner in LWW resolution

## Run / Test
```bash
cargo run -p multi-leader
cargo test -p multi-leader
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Writes survive DC failure | Conflicts require resolution logic |
| Low write latency per DC | LWW can silently discard writes |
| No single write bottleneck | Complex to reason about consistency |

## Real-World Usage
- **CouchDB** (MVCC + user-defined conflict resolution)
- **Cassandra** (LWW by default)
- **Git** (multi-leader with manual conflict resolution)

## Further Reading
- [Designing Data-Intensive Applications — Chapter 5 (Multi-Leader)](https://dataintensive.net/)
- [Riak Data Types](https://docs.riak.com/riak/kv/latest/developing/data-types/)
