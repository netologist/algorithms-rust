# Gossip Protocol

## What Is This Pattern?
**Gossip** (also called epidemic protocol) spreads information through a cluster
by having each node periodically share its state with a few random peers (fanout).
Within O(log N) rounds, all nodes converge to the same state — without any
central coordinator.

## When To Use It
- Cluster membership (Cassandra, Consul, Serf)
- Distributed failure detection
- Eventual consistency without a coordinator

## How It Works

```
Round 1: node-0 infects nodes 3, 7, 9
Round 2: nodes 0,3,7,9 each infect 3 more
Round 3: most nodes infected
...
Converges in ~log(N)/log(fanout) rounds
```

## Key Rust Concepts Used
- **`std::thread::spawn`**: per-node gossip background threads
- **`SliceRandom::shuffle`**: random peer selection each round
- **`Arc<Mutex<GossipNode>>`**: shared node state across threads

## Run / Test
```bash
cargo run -p gossip-protocol && cargo test -p gossip-protocol
```

## Real-World Usage
- **Apache Cassandra** (gossip for cluster membership + failure detection)
- **Consul** (Serf gossip layer)
- **Bitcoin** (transaction propagation)

## Further Reading
- [Epidemic Algorithms for Replicated Database Maintenance — Demers et al. (1987)](https://dl.acm.org/doi/10.1145/41840.41841)
