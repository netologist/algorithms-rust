# Rendezvous Hashing (Highest Random Weight)

## What Is This Pattern?
For each key, compute a score for every node as `hash(key + node_id)`. The node
with the **highest score** owns the key. When a node is removed, only its keys
are remapped — they go to whoever now has the highest score for each key.

## When To Use It
- CDN cache node selection
- Load balancing with minimal disruption on node changes
- Alternative to consistent hashing (no virtual nodes needed)

## How It Works

```
key="user:42", nodes = [A, B, C, D]
  score(key, A) = 0xFA12...
  score(key, B) = 0x2B99...  ← highest → B owns key
  score(key, C) = 0x8C44...
  score(key, D) = 0x1A22...
```

## Key Rust Concepts Used
- **`max_by_key`**: pick highest-scored node
- **`DefaultHasher`**: combines key and node into a score
- **No BTreeMap needed**: simpler than consistent hashing

## Run / Test
```bash
cargo run -p rendezvous-hashing && cargo test -p rendezvous-hashing
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Minimal remapping on node removal | O(N) per lookup (check all nodes) |
| No virtual nodes or ring structure | Slower than consistent hashing for large N |
| Provably minimal disruption | |

## Further Reading
- [Rendezvous Hashing — Thaler & Ravishankar (1998)](https://ieeexplore.ieee.org/document/663936)
