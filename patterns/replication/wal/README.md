# Write-Ahead Log (WAL)

## What Is This Pattern?
Before applying any change to the actual data store, append a record to an
**append-only log** first. On crash and restart, replay the log to reconstruct
the state. This guarantees **durability**: no committed write is lost even if
the process crashes mid-operation.

## When To Use It
- Any database or storage engine that needs durability
- Event sourcing (the WAL is the event log)
- Distributed systems that need crash recovery

## How It Works

```
Write("SET x 3")
   │
   ▼
[WAL: append entry, status=Committed] ──► safe to acknowledge
   │
   ▼
[Apply to in-memory store]

Crash recovery:
   WAL entries (Committed only) ──► replay ──► store rebuilt
```

Uncommitted entries (partial writes) are ignored on recovery.

## Key Rust Concepts Used
- **`Arc<Mutex<Vec<LogEntry>>>`**: append-only log shared across handles
- **`EntryStatus` enum**: distinguishes committed vs uncommitted
- **`splitn`**: parse "SET key value" operation strings

## Run / Test
```bash
cargo run -p wal
cargo test -p wal
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Durability without fsync on every op | Log grows unboundedly (needs compaction) |
| Crash recovery without user intervention | Replay time proportional to log length |
| Foundation for event sourcing | Uncommitted entries can confuse recovery |

## Real-World Usage
- **PostgreSQL** WAL, **SQLite** journal, **RocksDB** WAL
- **Kafka** (entire design is WAL-based)
- **etcd** Raft log

## Further Reading
- [The Log — Jay Kreps](https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying)
