# Leader Election (Bully Algorithm)

## What Is This Pattern?
**Leader Election** ensures exactly one node in a cluster acts as the coordinator.
This implementation uses the **Bully Algorithm**: when an election is triggered,
each candidate "bullies" lower-ID nodes by declaring itself winner; the node with
the highest alive ID always wins.

## When To Use It
- Distributed systems needing a single writer or primary (Kafka controller, Redis Sentinel)
- Cluster coordinator selection (Elasticsearch master)
- Any pattern that requires a single authoritative node (Primary-Replica, Raft)

## How It Works

```
Nodes: 1, 2, 3, 4, 5(leader)

Leader 5 crashes
   │
Node 4 detects absence → starts election
   ├─ Election ──► 5 (no response — dead)
   └─ No higher response → Node 4 wins
   │
   └─ Coordinator(4) ──► 1, 2, 3
```

**Steps:**
1. Node detects leader is absent (timeout or explicit signal).
2. Node sends `Election` to all nodes with higher IDs.
3. If no higher node responds within timeout → declares itself coordinator.
4. If a higher node responds → that node takes over the election.
5. Winner broadcasts `Coordinator(winner_id)` to all nodes.

**Simplification in this implementation:** Because we're in-process, we directly
find the maximum alive ID rather than exchanging timeout-based messages.

## Key Rust Concepts Used
- **`Arc<Mutex<Node>>`**: shared mutable node state across threads
- **Iterator combinators**: `.filter().map().max()` to find highest alive ID
- **RAII lock guards**: `n.lock().unwrap()` — auto-released at scope end

## Run
```bash
cargo run -p leader-election
```

## Test
```bash
cargo test -p leader-election
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Simple and deterministic | O(n²) messages in worst case |
| Highest-ID winner is predictable | Higher IDs always win — no load awareness |
| Self-healing on node failure | Network partition can elect two leaders |

## Real-World Usage
- **ZooKeeper** — ZAB leader election
- **Kafka** — controller election via ZooKeeper
- **Elasticsearch** — master election
- **Redis Sentinel** — Redis primary selection

## Further Reading
- [Garcia-Molina: Elections in a Distributed Computing System (1982)](https://www.cs.cornell.edu/courses/cs614/2003sp/papers/gm82.pdf)
- [Distributed Systems — Maarten van Steen (Chapter 6)](https://www.distributed-systems.net/index.php/books/ds4/)
