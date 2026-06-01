# Read Repair

## What Is This Pattern?
**Read Repair** fixes stale replicas lazily — at read time rather than at
write time. When a read detects that some replicas have an older version,
it asynchronously writes the latest value back to the stale nodes.

## When To Use It
- Leaderless replication (Dynamo-style systems)
- Complement to quorum reads — repairs staleness detected during reads
- Systems where write-time replication is best-effort

## How It Works

```
Read key from all 3 nodes:
  Node-0: ("v2", version=2) ← latest
  Node-1: ("v2", version=2)
  Node-2: ("v1", version=1) ← STALE

Return "v2" to caller.
In background: write ("v2", version=2) to Node-2.
```

Repair is **asynchronous** — the read returns immediately with the
latest value; the background write fixes the stale replica.

## Key Rust Concepts Used
- **`thread::spawn` for background repair**: non-blocking repair
- **`max_by_key(|v| v.version)`**: detect latest value
- **Stale detection**: compare each node's version to the maximum

## Run / Test
```bash
cargo run -p read-repair
cargo test -p read-repair
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Repairs happen without extra coordination | Only runs when a key is read |
| Read latency unaffected (repair is async) | Rarely-read keys stay stale indefinitely |
| Simple to implement | Background writes may spike under heavy reads |

## Real-World Usage
- **Apache Cassandra** (read repair on every read, configurable probability)
- **Amazon Dynamo** (read repair + anti-entropy)
- **Riak** read repair

## Further Reading
- [Designing Data-Intensive Applications — Chapter 5](https://dataintensive.net/)
- [Cassandra Read Repair docs](https://cassandra.apache.org/doc/latest/cassandra/operations/read_repair.html)
