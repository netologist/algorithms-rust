# Range Partitioning

## What Is This Pattern?
Keys are divided into **sorted ranges**, each assigned to a shard. A shard is
responsible for all keys between its `min` (inclusive) and `max` (exclusive).
Range-based lookups and scans are efficient — they stay within one shard.

## When To Use It
- Time-series data (shard by date range)
- User data with lexicographic distribution
- Any workload with frequent range scans

## How It Works

```
Shard-A: [-, f)   → apple, cherry, date
Shard-B: [f, p)   → fig, kiwi, orange
Shard-C: [p, +)   → pear, tomato, zebra
```

## Key Rust Concepts Used
- **String comparison (`>=`, `<`)**: range checks on string keys
- **`position`**: find owning shard index
- **Builder-style `Vec<Shard>`**: flexible range configuration

## Run / Test
```bash
cargo run -p range-partitioning && cargo test -p range-partitioning
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Range scans stay in one shard | Hot spots if keys cluster in one range |
| Ordered iteration within shard | Rebalancing requires key migration |
| Easy to understand | Keys must be sortable |

## Further Reading
- [Designing Data-Intensive Applications — Chapter 6](https://dataintensive.net/)
- [BigTable Paper — Chang et al.](https://research.google/pubs/bigtable-a-distributed-storage-system-for-structured-data/)
