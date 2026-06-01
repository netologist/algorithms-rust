# Saga (Choreography)

## What Is This Pattern?
A **saga** breaks a long-running distributed transaction into local transactions,
each with a compensating transaction to undo it on failure.

In **choreography**, services communicate only through domain events — no
central coordinator. Each service subscribes to events, does its work, and
publishes its own events. Compensation is triggered by failure events.

## When To Use It
- Distributed transactions spanning multiple microservices
- When a central coordinator would create a bottleneck or coupling
- Order processing, travel booking, financial workflows

## How It Works

```
OrderService           PaymentService        InventoryService
     │ OrderPlaced ──────────────────►│
     │                PaymentProcessed│──────────────►│
     │                                │ InventoryReserved
     │◄─────────────────────────────────────────────── OrderCompleted

Failure:
     │ OrderPlaced ──────────────────►│
     │                   PaymentFailed│
     │◄──── OrderCancelled ───────────│  (compensating tx)
```

## Key Rust Concepts Used
- **`VecDeque<SagaEvent>`**: event bus FIFO queue
- **Pattern matching on event enum**: drives the saga state machine
- **History tracking**: `Vec<SagaEvent>` for test assertions

## Run / Test
```bash
cargo run -p saga-choreography && cargo test -p saga-choreography
```

## Trade-offs
| Pro | Con |
|-----|-----|
| No central coordinator bottleneck | Hard to track overall saga state |
| Services loosely coupled | Difficult to add new steps |
| Each service independently deployable | Compensating transactions must be idempotent |

## Further Reading
- [Saga Pattern — Chris Richardson](https://microservices.io/patterns/data/saga.html)
