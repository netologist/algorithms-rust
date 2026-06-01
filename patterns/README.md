# Distributed Systems Patterns

Each subdirectory is a **category** of related patterns. Inside each category,
every pattern is a standalone Rust crate with its own `README.md`, runnable
demo, and tests.

## Categories

| Category | Patterns | Description |
|----------|----------|-------------|
| [Consensus](consensus/) | 5 | Agreeing on values across unreliable nodes |
| [Discovery](discovery/) | 3 | Locating services and distributing load |
| [Fault Tolerance](fault-tolerance/) | 5 | Surviving failures and slowdowns |
| [Gossip](gossip/) | 3 | Decentralized peer-to-peer communication |
| [Messaging](messaging/) | 6 | Reliable, decoupled service communication |
| [Observability](observability/) | 3 | Monitoring health, detecting failures, tracing requests |
| [Partitioning](partitioning/) | 3 | Distributing data across nodes (sharding) |
| [Replication](replication/) | 6 | Copying data for durability and availability |

## How to Explore

```bash
# Run a pattern's demo
cargo run -p circuit-breaker

# Test a pattern
cargo test -p raft

# Build everything
make build
```
