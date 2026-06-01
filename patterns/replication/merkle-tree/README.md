# Merkle Tree

A **hash tree** used to verify data integrity and efficiently synchronise replicated datasets.

## How it Works

```
          ROOT
         /    \
       H(AB)  H(CD)
       / \    / \
      HA  HB HC  HD      ← leaf hashes = SHA-256(data)
      |   |  |   |
     "a" "b" "c" "d"     ← raw data blocks
```

1. Hash each leaf: `leaf_hash = SHA-256(data)`.
2. Pair adjacent hashes and hash the pair: `parent = SHA-256(left ‖ right)`.
3. If a level has an odd count, duplicate the last node.
4. Repeat until one root hash remains.

## Use Cases in Distributed Systems

* **Anti-entropy** (Cassandra, DynamoDB) — compare root hashes across replicas.
  If they differ, walk the tree to find the diverged subtree in `O(log n)` comparisons.
* **Git / Merkle DAG** — content-addressable object store.
* **Blockchain** — Merkle root in block headers lets light clients verify transactions.
* **Distributed file systems** (IPFS, BitTorrent) — verify individual pieces without the full file.

## API

| Method | Description |
|--------|-------------|
| `MerkleTree::new(data)` | Build tree from byte payloads |
| `root()` | Root hash (`None` for empty tree) |
| `leaves()` | Leaf hashes in order |
| `verify_leaf(i, data)` | Check that `data` still matches leaf `i` |
| `matches(other)` | Compare roots between two trees |

## Complexity

| Operation | Time | Space |
|-----------|------|-------|
| Build     | O(n) | O(n)  |
| Root compare | O(1) | — |
| Subtree diff | O(log n) | — |

## Running

```bash
cargo run   -p merkle-tree
cargo test  -p merkle-tree
```
