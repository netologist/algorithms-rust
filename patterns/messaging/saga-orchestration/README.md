# Saga (Orchestration)

## What Is This Pattern?
In **orchestration**, a central **orchestrator** drives each step of the saga
in sequence. On failure, it runs compensating transactions for all completed
steps in reverse order.

Unlike choreography, the orchestrator has full visibility into the saga state.

## When To Use It
- Complex multi-step workflows where visibility matters
- When you need central monitoring/retry/timeout logic
- Long-running business processes (order fulfillment, loan approval)

## How It Works

```
Orchestrator
  → Step 1: Reserve Inventory  ✓
  → Step 2: Charge Payment     ✗ (FAILED)
  ← Compensate Step 1: Release Inventory
  → Saga: RolledBack
```

## Key Rust Concepts Used
- **`Box<dyn Fn()>`**: dynamic dispatch for step execute/compensate closures
- **`Vec` of completed steps**: reverse iteration for rollback
- **Builder pattern**: `add_step(self, step) -> Self`

## Run / Test
```bash
cargo run -p saga-orchestration && cargo test -p saga-orchestration
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Central visibility into saga state | Orchestrator is a single point of failure |
| Easy to add retry/timeout logic | Tighter coupling to orchestrator |
| Clear rollback logic | Orchestrator can become a God Object |

## Further Reading
- [Saga Pattern — Chris Richardson](https://microservices.io/patterns/data/saga.html)
- [Designing Distributed Systems — Brendan Burns](https://www.oreilly.com/library/view/designing-distributed-systems/9781491983638/)
