# Gossip Patterns

Decentralized protocols where nodes periodically exchange state with random
peers, achieving eventual consistency without a central coordinator.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Gossip Protocol](gossip-protocol/) | `gossip-protocol` | Nodes periodically share state with random peers; state converges across the cluster |
| [Epidemic Broadcast](epidemic-broadcast/) | `epidemic-broadcast` | SI model — initially one infected node; message spreads to all susceptible nodes |
| [Chord DHT](chord-dht/) | `chord-dht` | Distributed hash table with O(log N) lookup via finger tables and consistent hashing ring |

## Key Rust Concepts

- Async tasks (`tokio::spawn`) for per-node gossip loops
- Random peer selection with `rand::Rng`
- Finger-table routing for logarithmic DHT lookups
- `Arc<Mutex<T>>` for shared node state across concurrent gossip tasks
