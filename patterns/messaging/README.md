# Messaging Patterns

Patterns for reliable, decoupled communication between services —
covering message delivery, event-driven state, and distributed transactions.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Pub-Sub](pub-sub/) | `pub-sub` | Publishers emit messages to topics; subscribers receive only what they're interested in |
| [Event Sourcing](event-sourcing/) | `event-sourcing` | Append-only event log as the source of truth; current state is a projection of events |
| [CQRS](cqrs/) | `cqrs` | Separate read and write models — commands mutate, queries project from events |
| [Saga (Choreography)](saga-choreography/) | `saga-choreography` | Distributed transaction where each step publishes the next event (no central coordinator) |
| [Saga (Orchestration)](saga-orchestration/) | `saga-orchestration` | Distributed transaction with a central orchestrator that sequences and compensates steps |
| [Transactional Outbox](outbox/) | `outbox` | Atomically write events to an outbox table alongside business data; a relay publishes them |

## Key Rust Concepts

- Event log as `Vec<Event>` with append-only semantics
- Trait-based read/write model separation (CQRS)
- Channel-based pub/sub with topic routing
- Async saga orchestration with compensation handlers
