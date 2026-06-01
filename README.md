# Distributed Systems Patterns in Rust

Reference implementations of classic distributed systems patterns, written in
idiomatic Rust. Each pattern is a self-contained crate with runnable demos and
tests — no external services required.

## Structure

```
.
├── common/                  Shared simulation primitives (network, messaging, clocks)
├── patterns/
│   ├── consensus/           Leader election, 2PC, 3PC, Paxos, Raft
│   ├── discovery/           Service registry, load balancer, sidecar
│   ├── fault-tolerance/     Circuit breaker, retry/backoff, bulkhead, timeout, fallback
│   ├── gossip/              Gossip protocol, epidemic broadcast, Chord DHT
│   ├── messaging/           Pub/sub, event sourcing, CQRS, saga, outbox
│   ├── observability/       Heartbeat, health check, distributed tracing
│   ├── partitioning/        Consistent hashing, rendezvous hashing, range partitioning
│   └── replication/         Primary/replica, multi-leader, WAL, CRDT, quorum, read repair
├── Cargo.toml               Workspace root
└── Makefile                 Build, test, lint helpers
```

## Quick Start

```bash
# Build everything
make build

# Run all tests
make test

# Run a specific pattern
cargo run -p circuit-breaker

# Lint
make lint
```

## Patterns

### Consensus
| Pattern | Description |
|---------|-------------|
| [leader-election](patterns/consensus/leader-election/) | Bully algorithm leader election |
| [two-phase-commit](patterns/consensus/two-phase-commit/) | 2PC atomic commit protocol |
| [three-phase-commit](patterns/consensus/three-phase-commit/) | 3PC non-blocking commit |
| [paxos](patterns/consensus/paxos/) | Classic Paxos consensus |
| [raft](patterns/consensus/raft/) | Raft consensus (understandable alternative to Paxos) |

### Discovery
| Pattern | Description |
|---------|-------------|
| [service-registry](patterns/discovery/service-registry/) | Dynamic service registration and lookup |
| [load-balancer](patterns/discovery/load-balancer/) | Client-side and server-side load balancing |
| [sidecar](patterns/discovery/sidecar/) | Process-level proxy for cross-cutting concerns |

### Fault Tolerance
| Pattern | Description |
|---------|-------------|
| [circuit-breaker](patterns/fault-tolerance/circuit-breaker/) | Fail-fast on failing downstream services |
| [retry-backoff](patterns/fault-tolerance/retry-backoff/) | Exponential backoff with jitter |
| [bulkhead](patterns/fault-tolerance/bulkhead/) | Resource isolation with bounded concurrency |
| [timeout](patterns/fault-tolerance/timeout/) | Bounded wait with graceful degradation |
| [fallback](patterns/fault-tolerance/fallback/) | Graceful degradation with alternative responses |

### Gossip
| Pattern | Description |
|---------|-------------|
| [gossip-protocol](patterns/gossip/gossip-protocol/) | Epidemic-style state dissemination |
| [epidemic-broadcast](patterns/gossip/epidemic-broadcast/) | SI model — infected/susceptible broadcast |
| [chord-dht](patterns/gossip/chord-dht/) | Chord distributed hash table |

### Messaging
| Pattern | Description |
|---------|-------------|
| [pub-sub](patterns/messaging/pub-sub/) | Publish-subscribe messaging |
| [event-sourcing](patterns/messaging/event-sourcing/) | State as a sequence of immutable events |
| [cqrs](patterns/messaging/cqrs/) | Separate read and write models |
| [saga-choreography](patterns/messaging/saga-choreography/) | Distributed transactions via events |
| [saga-orchestration](patterns/messaging/saga-orchestration/) | Distributed transactions via coordinator |
| [outbox](patterns/messaging/outbox/) | Reliable message publication |

### Observability
| Pattern | Description |
|---------|-------------|
| [heartbeat](patterns/observability/heartbeat/) | Periodic liveness signals |
| [health-check](patterns/observability/health-check/) | On-demand health probes |
| [distributed-tracing](patterns/observability/distributed-tracing/) | End-to-end request tracing |

### Partitioning
| Pattern | Description |
|---------|-------------|
| [consistent-hashing](patterns/partitioning/consistent-hashing/) | Ring-based key distribution |
| [rendezvous-hashing](patterns/partitioning/rendezvous-hashing/) | Highest random weight hashing |
| [range-partitioning](patterns/partitioning/range-partitioning/) | Key-range based sharding |

### Replication
| Pattern | Description |
|---------|-------------|
| [primary-replica](patterns/replication/primary-replica/) | Single-primary replication |
| [multi-leader](patterns/replication/multi-leader/) | Multi-primary with conflict resolution |
| [wal](patterns/replication/wal/) | Write-ahead log for durability |
| [crdt](patterns/replication/crdt/) | Conflict-free replicated data types |
| [quorum](patterns/replication/quorum/) | Majority-based read/write |
| [read-repair](patterns/replication/read-repair/) | Lazy consistency repair on read |

## Requirements

- **Rust** 1.75+ (edition 2021)
- `cargo` (comes with Rust via [rustup](https://rustup.rs))

## License

This project is for learning and reference. See individual crates for details.
