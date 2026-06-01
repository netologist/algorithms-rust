# Observability Patterns

Patterns for monitoring system health, detecting failures, and tracing
requests across service boundaries.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Heartbeat](heartbeat/) | `heartbeat` | Nodes periodically send liveness signals; absence triggers failure detection |
| [Health Check](health-check/) | `health-check` | On-demand endpoint that reports service status (healthy/degraded/unhealthy) |
| [Distributed Tracing](distributed-tracing/) | `distributed-tracing` | Trace ID propagation across services; spans form a call tree for debugging latency |

## Key Rust Concepts

- `tokio::time::interval` for periodic heartbeat ticks
- `tracing` crate for structured, span-based logging
- Span context propagation via message headers
- `HashMap`-based health registry with TTL-based expiration
