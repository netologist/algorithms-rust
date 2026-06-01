# Heartbeat

## What Is This Pattern?
A **heartbeat** is a periodic signal that proves a node is alive. If the
monitor doesn't receive a heartbeat within a configurable timeout, it marks
the node as **Down**. This is the simplest form of failure detection.

## When To Use It
- Detecting crashed nodes in a cluster
- Leader failure detection (trigger re-election)
- Health monitoring in distributed systems

## How It Works

```
Node ──beat()──► Monitor  (every 100ms)

Monitor checks:
  now - last_beat(node) ≤ timeout → Up
  now - last_beat(node) > timeout → Down
```

## Key Rust Concepts Used
- **`Instant::elapsed()`**: time since last beat without atomics
- **`HashMap<NodeId, Instant>`**: per-node last-seen timestamp
- **`Beat + status`**: decoupled send and check

## Run / Test
```bash
cargo run -p heartbeat && cargo test -p heartbeat
```

## Real-World Usage
- **ZooKeeper** session timeouts
- **Raft** election timeout (absence of heartbeat triggers election)
- **Kubernetes** node `Ready` condition via kubelet

## Further Reading
- [Failure Detectors — Fischer, Lynch, Paterson](https://en.wikipedia.org/wiki/Failure_detector)
