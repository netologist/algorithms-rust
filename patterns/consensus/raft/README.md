# Raft

## What Is This Pattern?
**Raft** is a distributed consensus algorithm designed to be understandable. It
ensures that a cluster of nodes agrees on an ordered sequence of log entries even
when some nodes fail. One node is the **Leader** — it receives all writes and
replicates them to **Followers**. A write is committed once a majority (quorum)
acknowledges it.

## When To Use It
- Replicated state machines (databases, distributed KV stores)
- Distributed lock services (etcd, Consul)
- Coordinating cluster membership

## How It Works

```
        [Client]
           │ write
           ▼
       [Leader]  ──heartbeat──►  [Follower 1]
           │                     [Follower 2]
           │ AppendEntries       [Follower 3]
           └──────────────────►  [Follower 4]

Leader Election:
  Follower times out ──► Candidate ──► broadcasts RequestVote
  Majority votes ──► Leader
```

**Three sub-problems:**

1. **Leader Election**: Followers start elections on timeout. The candidate with
   the most up-to-date log and majority votes wins.

2. **Log Replication**: Leader appends entry, sends `AppendEntries` to all peers.
   Entry committed once majority ACKs.

3. **Safety**: A leader can only be elected if its log is at least as up-to-date
   as any voter's log. No two leaders exist in the same term.

## This Implementation
- Each node runs in a `tokio::spawn` task with an `mpsc::unbounded_channel` inbox
- `RaftRole` enum: Follower / Candidate / Leader
- Election timeout: random `[150ms, 300ms]` — avoids split votes
- Heartbeat interval: 50ms — leader sends empty `AppendEntries` to suppress elections
- Simplified: commit tracking uses optimistic majority count; no log matching index

## Key Rust Concepts Used
- **`tokio::time::timeout`**: implements the election/heartbeat timers
- **`Arc<Mutex<NodeState>>`**: shared node state across tasks
- **`mpsc::unbounded_channel`**: per-node message inbox
- **Pattern matching on enum messages**: clean Raft message dispatch

## Run
```bash
cargo run -p raft
```

## Test
```bash
cargo test -p raft
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Designed to be understandable | More complex than single-master replication |
| Strong consistency guarantees | Write throughput limited by leader |
| Handles leader failure automatically | Network partitions require quorum — minority partitions block |

## Real-World Usage
- **etcd** — Raft for Kubernetes cluster state
- **CockroachDB**, **TiKV** — Raft for shard consensus
- **Consul** — leader election and KV store
- **raft-rs** crate (TiKV's Raft implementation in Rust)

## Further Reading
- [Raft Paper — Ongaro & Ousterhout (2014)](https://raft.github.io/raft.pdf)
- [The Raft Website](https://raft.github.io/)
- [Raft Visualization](https://raft.github.io/)
