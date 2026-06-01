# Event Sourcing

## What Is This Pattern?
Instead of storing current state, store an **append-only log of events** that
describe what happened. Current state is derived by **replaying** the events.
A **snapshot** + partial replay avoids replaying the entire history on startup.

## When To Use It
- Audit trail required (financial transactions, medical records)
- Time-travel debugging (replay up to any point)
- CQRS read models (project events into different views)
- Undo/redo functionality

## How It Works

```
Events: [Deposited(100), Withdrawn(30), Deposited(50)]
Replay: 0 + 100 - 30 + 50 = 120  ← current balance

Snapshot at event 2: balance=70
Partial replay events[2..]: + 50 = 120  ← same result, cheaper
```

## Key Rust Concepts Used
- **`Vec<AccountEvent>` append-only**: enforced by not exposing `&mut` directly
- **`AccountState::from_events`**: pure fold over event slice
- **`events_from(idx)`**: slice-based partial replay
- **`derive(Clone)`**: snapshot clones the state

## Run / Test
```bash
cargo run -p event-sourcing && cargo test -p event-sourcing
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Full audit history | Log grows unboundedly without compaction |
| Time-travel replay | Rebuilding state from scratch can be slow |
| Decoupled read models | Schema changes require event migration |

## Real-World Usage
- **Kafka** (event log as system of record)
- **EventStoreDB** (purpose-built event sourcing DB)
- **Axon Framework** (Java CQRS+ES)

## Further Reading
- [Martin Fowler — Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)
- [Greg Young — CQRS and Event Sourcing (video)](https://www.youtube.com/watch?v=JHGkaShoyNs)
