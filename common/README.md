# Common — Shared Simulation Primitives

The `common` crate provides the building blocks used by all pattern demos:
a simulated network, message passing, logical clocks, and node abstractions.

## What's Inside

| Module | Purpose |
|--------|---------|
| `SimNode` | A network node with an inbox channel (`Sender`/`Receiver`) |
| `NetworkSim` | In-memory network bus with configurable latency and drop rate |
| `Message` | Generic envelope with serde-encoded payload and Lamport timestamp |
| `LogicalClock` | Lamport logical clock — tick on send, merge on receive |
| `NodeId` | Unique node identifier |
| `random_id()` | Generate a random `NodeId` |
| `sleep_ms()` | Convenience sleep helper |

## Usage

```rust
use common::{SimNode, NetworkSim, Message, LogicalClock};

let node_a = SimNode::new(1);
let node_b = SimNode::new(2);

let mut net = NetworkSim::new()
    .with_latency(10)      // 10ms delivery delay
    .with_drop_rate(0.05); // 5% packet loss

net.register(&node_a);
net.register(&node_b);

// Send a message
net.send(Message::new(1, 2, LogicalClock(1), &"hello"));
```

## Key Rust Concepts Used

- **`crossbeam_channel`**: multi-producer, multi-consumer channels for inboxes
- **`serde` / `serde_json`**: generic payload encoding
- **`rand`**: random IDs and drop-rate simulation
- **`std::thread::spawn`**: simulated network latency runs on background threads
