# Discovery Patterns

Mechanisms for locating services, distributing load, and managing configuration
in a dynamic distributed system.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Service Registry](service-registry/) | `service-registry` | Central directory where services register and clients look up endpoints |
| [Load Balancer](load-balancer/) | `load-balancer` | Distributes requests across multiple backend instances (round-robin, least-connections) |
| [Sidecar](sidecar/) | `sidecar` | Companion process that offloads cross-cutting concerns (logging, proxying, config) from the application |

## Key Rust Concepts

- Synchronous `crossbeam` channels for service registry communication
- Trait-based load-balancing strategies
- `HashMap`-backed registry with health tracking
