# Consistent Hashing

## What Is This Pattern?
**Consistent hashing** places both nodes and keys on a virtual ring. A key is owned by
the first node clockwise from its hash position. When nodes are added or removed,
only keys near that node are remapped — all other keys stay put.

## When To Use It
- Distributed caches (Memcached, Redis Cluster)
- Load balancing with session affinity
- Sharding in databases (Cassandra, Dynamo)

## How It Works

```
Ring (0..2^64):
  key-7 → [node-A at 10] → owned by node-A
  key-3 → [node-C at 95] → wraps to node-A (first clockwise)

Virtual nodes: each physical node maps to V positions
→ more even distribution, fewer keys remapped on changes
```

## Key Rust Concepts Used
- **`BTreeMap<u64, String>`**: sorted ring — enables O(log N) clockwise lookup
- **`range(h..).next()`**: find first key ≥ h on the ring
- **`DefaultHasher`**: consistent hash function

## Run / Test
```bash
cargo run -p consistent-hashing && cargo test -p consistent-hashing
```

## Trade-offs
| Pro | Con |
|-----|-----|
| O(1/N) keys remapped on node change | Uneven distribution without virtual nodes |
| O(log N) lookup in BTreeMap | Ring size limits key space |
| No global rehash | Virtual nodes add memory overhead |

## Further Reading
- [Consistent Hashing and Random Trees — Karger et al. (1997)](https://dl.acm.org/doi/10.1145/258533.258660)
- [Amazon Dynamo Paper (2007)](https://dl.acm.org/doi/10.1145/1294261.1294281)
