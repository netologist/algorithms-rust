# Partitioning Patterns

Strategies for distributing data across multiple nodes (sharding) so that
each node owns a subset of the keyspace.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Consistent Hashing](consistent-hashing/) | `consistent-hashing` | Ring-based hashing — add/remove nodes with minimal key reassignment |
| [Rendezvous Hashing](rendezvous-hashing/) | `rendezvous-hashing` | Highest Random Weight — each key hashes with every node; node with highest score wins |
| [Range Partitioning](range-partitioning/) | `range-partitioning` | Keys sorted into contiguous ranges; each range assigned to a specific node |

## Key Rust Concepts

- `BTreeMap` for ordered range lookups (range partitioning)
- `BinaryHeap` / sorting for rendezvous hash scoring
- `std::hash::Hasher` for deterministic key placement
- Generic `Key`/`Node` types for flexible sharding
