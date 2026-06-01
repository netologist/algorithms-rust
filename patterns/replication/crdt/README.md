# CRDT (Conflict-Free Replicated Data Types)

## What Is This Pattern?
**CRDTs** are data structures with a mathematically guaranteed merge operation:
any two replicas can merge their states and converge to the same result without
coordination. No consensus needed. No conflicts possible.

This implements two CRDTs:
- **G-Counter** (Grow-only counter)
- **LWW-Register** (Last-Write-Wins register)

## When To Use It
- Collaborative editing (shared counters, presence, cursor positions)
- Distributed caches (merge without coordination)
- Systems that must remain available during network partitions (AP systems)

## How It Works

**G-Counter merge:**
```
Node-1: {1: 5, 2: 0, 3: 0}
Node-2: {1: 0, 2: 3, 3: 0}
Merge:  {1: 5, 2: 3, 3: 0}  → value = 5+3 = 8
Rule: max(local[i], remote[i]) per slot
```

**LWW-Register merge:**
```
r1: ("hello", ts=100)
r2: ("world", ts=200)
Merge: ("world", ts=200)  → higher ts wins
```

**CRDT properties:**
- **Commutative**: merge(A,B) = merge(B,A)
- **Associative**: merge(merge(A,B),C) = merge(A,merge(B,C))
- **Idempotent**: merge(A,A) = A

## Key Rust Concepts Used
- **`HashMap<NodeId, u64>`**: per-slot counter storage
- **`entry().or_insert(0)`**: insert default if absent
- **`max_by_key`**: LWW winner selection
- **`derive(Clone)`**: CRDTs must be cloneable for merge

## Run / Test
```bash
cargo run -p crdt
cargo test -p crdt
```

## Trade-offs
| Pro | Con |
|-----|-----|
| No coordination needed | Not all data types have CRDT versions |
| Eventual consistency guaranteed mathematically | LWW silently discards concurrent writes |
| Works across network partitions | State-based CRDTs can have large payloads |

## Real-World Usage
- **Riak** (CRDT-native distributed database)
- **Redis CRDB** (Conflict-free Replicated Data Base)
- **Figma** (operational transforms / CRDTs for collaborative design)

## Further Reading
- [A comprehensive study of CRDTs — Shapiro et al. (2011)](https://hal.inria.fr/inria-00555588/document)
- [CRDTs: The Hard Parts — Martin Kleppmann](https://www.youtube.com/watch?v=x7drE24geUw)
