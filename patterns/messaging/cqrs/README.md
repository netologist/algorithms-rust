# CQRS (Command Query Responsibility Segregation)

## What Is This Pattern?
**CQRS** separates the write model (commands → events) from the read model
(projections). Commands mutate state via the `CommandHandler`; queries read
from a `Projection` that is updated by applying events.

## When To Use It
- Different read and write scalability requirements
- Multiple read-optimised views of the same data
- Complex domain logic that benefits from explicit command/event vocabulary
- Combined with event sourcing

## How It Works

```
Client
  │ Command(CreateOrder)
  ▼
CommandHandler ──► DomainEvent(OrderCreated) ──► EventStore
                                              ──► Projection(update)
  │ Query(allActive)
  ▼
OrderProjection ──► [OrderView, OrderView, ...]
```

## Key Rust Concepts Used
- **Enum commands and events**: explicit vocabulary for domain actions
- **`HashSet<u64>` for ID tracking**: O(1) duplicate detection
- **Separate structs for write/read sides**: enforces separation

## Run / Test
```bash
cargo run -p cqrs && cargo test -p cqrs
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Read/write models optimised independently | Added complexity (two models to maintain) |
| Explicit domain vocabulary | Eventual consistency between write and read side |
| Easy to add new read projections | Overkill for simple CRUD |

## Real-World Usage
- **Axon Framework**, **EventStoreDB**
- **Microsoft Azure** CQRS guidance

## Further Reading
- [Martin Fowler — CQRS](https://martinfowler.com/bliki/CQRS.html)
