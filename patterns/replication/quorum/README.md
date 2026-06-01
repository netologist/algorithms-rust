# Quorum

## What Is This Pattern?
A **quorum** is a minimum number of nodes that must agree before an operation
succeeds. Using W (write quorum) and R (read quorum) such that **W + R > N**
(total nodes) guarantees that every read will see at least one node that
received the most recent write.

## When To Use It
- Leaderless distributed databases (Cassandra, Dynamo)
- Tunable consistency — choose strong vs eventual per-request
- Replication with configurable fault tolerance

## How It Works

```
N=5 nodes, W=3, R=3 (W+R=6 > N=5)

Write("v1") to nodes {0, 1, 2}
Read from nodes {1, 2, 3} → at least 2 of 3 have v1 → return v1

Overlap guaranteed: W + R > N → at least 1 node in both sets
```

**Version numbers** break ties: the highest version wins on read.

## Key Rust Concepts Used
- **`SliceRandom::shuffle`**: random node selection per operation
- **`max_by_key`**: version-based winner on read
- **`Arc<Mutex<HashMap>>`**: thread-safe per-node store

## Run / Test
```bash
cargo run -p quorum
cargo test -p quorum
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Tunable consistency/availability tradeoff | Random node selection can give stale reads at low W+R |
| No single leader bottleneck | Coordination overhead per operation |
| Can tolerate up to (N-W) write failures | Stale reads possible when W+R ≤ N |

## Real-World Usage
- **Amazon Dynamo** / **DynamoDB** (configurable W+R)
- **Apache Cassandra** (ONE/QUORUM/ALL consistency levels)
- **Riak** (N/W/R tunable)

## Further Reading
- [Dynamo: Amazon's Highly Available Key-value Store (2007)](https://dl.acm.org/doi/10.1145/1294261.1294281)
- [Designing Data-Intensive Applications — Chapter 5](https://dataintensive.net/)
