# Replication Patterns

Techniques for copying data across multiple nodes to improve durability,
availability, and read throughput.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Primary-Replica](primary-replica/) | `primary-replica` | One primary accepts writes, replicates to read-only replicas; simple but single-writer |
| [Multi-Leader](multi-leader/) | `multi-leader` | Multiple nodes accept writes; requires conflict resolution (LWW, version vectors) |
| [Write-Ahead Log](wal/) | `wal` | Append to a log before applying to state; enables crash recovery |
| [CRDT](crdt/) | `crdt` | Conflict-Free Replicated Data Types — merge any two replicas and they converge |
| [Quorum](quorum/) | `quorum` | W + R > N — tune read/write quorums for consistency vs. availability |
| [Read Repair](read-repair/) | `read-repair` | On read, detect stale replicas and repair them lazily |

## Key Rust Concepts

- Append-only `Vec<Entry>` for write-ahead logs
- CRDT state as algebraic types with commutative merge operations
- Quorum-based `HashMap` read/write with version tracking
- `Arc`-shared replica state for concurrent access
