# Primary-Replica Replication

## What Is This Pattern?
One node (**Primary**) accepts all writes and asynchronously propagates them to
one or more **Replicas**. Replicas serve read requests and act as hot standbys.
This is the most common replication topology in production databases.

## When To Use It
- Read-heavy workloads (scale reads across replicas)
- Geo-distribution (replicas closer to readers)
- High-availability (promote replica on primary failure)

## How It Works

```
Client → Write → [Primary] ──async──► [Replica 1]
                           ──async──► [Replica 2]
Client → Read  → [Replica 1]  (or Primary)
```

1. Write arrives at Primary.
2. Primary applies write to its own store immediately.
3. Primary sends replication message to all replicas asynchronously.
4. Replicas apply the write (eventually consistent).

## Key Rust Concepts Used
- **`crossbeam_channel::unbounded`**: async replication queue
- **`Arc<Mutex<HashMap>>`**: shared store across threads
- **Background thread**: dedicated replication worker drains the channel

## Run / Test
```bash
cargo run -p primary-replica
cargo test -p primary-replica
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Simple mental model | Replica reads may be stale (eventual consistency) |
| Scales read throughput | Single write path is a bottleneck |
| Easy failover (promote replica) | Replication lag under high write load |

## Real-World Usage
- **PostgreSQL** streaming replication, **MySQL** binlog replication
- **Redis** primary-replica replication
- **MongoDB** replica sets

## Further Reading
- [Designing Data-Intensive Applications — Chapter 5](https://dataintensive.net/)
