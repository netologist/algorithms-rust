# Bloom Filter

A **space-efficient probabilistic data structure** for set-membership tests.

## How it Works

A Bloom filter maintains a bit array of length `m` and uses `k` independent hash functions.

* **Insert** an element → set `k` bit positions.
* **Query** an element → check all `k` positions.
  * All set → **probably present** (may be false positive).
  * Any unset → **definitely absent** (no false negatives).

```
insert("apple")  → hash₀("apple")=3, hash₁("apple")=7, hash₂("apple")=14
                    bit[3]=1, bit[7]=1, bit[14]=1

query("apple")   → bit[3]? bit[7]? bit[14]? → all 1 → POSSIBLY IN SET
query("durian")  → hash₀("durian")=5 → bit[5]=0 → DEFINITELY NOT IN SET
```

## Trade-offs

| Parameter | Effect |
|-----------|--------|
| Larger `m` | Lower false-positive rate, more memory |
| More `k` hashes | Lower FPR up to optimum, then increases |
| More elements | Higher fill → higher FPR |

**Optimal parameters** for `n` expected elements and target FPR `p`:
```
m = -n · ln(p) / (ln 2)²
k =  m / n · ln(2)
```

## Use Cases in Distributed Systems

* **Cache layer** — check bloom filter before hitting a DB/cache to avoid unnecessary lookups.
* **CDN / proxy** — avoid caching one-hit-wonder objects.
* **HBase / Cassandra / RocksDB** — skip SSTables that cannot contain a key.
* **Network routing** — IP reputation / blocklist lookups.

## Complexity

| Operation | Time | Space |
|-----------|------|-------|
| Insert    | O(k) | O(m)  |
| Query     | O(k) | O(m)  |

## Running

```bash
cargo run   -p bloom-filter
cargo test  -p bloom-filter
```
