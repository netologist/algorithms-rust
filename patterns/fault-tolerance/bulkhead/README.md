# Bulkhead

## What Is This Pattern?
Named after the watertight compartments in a ship's hull — if one floods, the
others stay dry. In software, a **bulkhead** limits the number of concurrent
callers that can access a resource. When the limit is reached, excess requests
are rejected immediately rather than queuing up and consuming resources
indefinitely.

## When To Use It
- Isolating thread pools between downstream services (slow service A can't starve service B)
- Limiting concurrent database connections per tenant
- Protecting a rate-limited external API from overload
- Preventing one slow endpoint from monopolising a shared thread pool

## How It Works

```
Incoming requests → [Bulkhead: 3 slots]
   Req 1 → slot 1 ████████ (holds until done)
   Req 2 → slot 2 ████████
   Req 3 → slot 3 ████████
   Req 4 → REJECTED (no free slot)
   Req 5 → REJECTED
```

1. `Bulkhead::new(capacity)` creates a semaphore with `capacity` permits.
2. `acquire()` decrements the counter and returns a `Permit` if available,
   or returns `Err(BulkheadFull)` immediately.
3. `Permit` implements `Drop` — the slot is automatically returned when the
   guard goes out of scope, even on panic.

## This Implementation
- `Bulkhead` wraps `Arc<Mutex<usize>>` for the available-permit counter.
- `Permit` is an RAII guard — no manual release needed.
- `try_acquire` is non-blocking: rejects instantly when full.

## Key Rust Concepts Used
- **RAII `Permit`**: `Drop` impl returns slot without manual cleanup
- **`Arc<Mutex<usize>>`**: shared mutable counter across threads
- **Non-blocking rejection**: returns `Err` immediately (no blocking wait)

## Run
```bash
cargo run -p bulkhead
```

## Test
```bash
cargo test -p bulkhead
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Prevents one consumer from starving others | Requires capacity tuning per resource |
| RAII ensures no slot leaks | Does not prioritise requests (FIFO would need a queue) |
| Zero async overhead | Blocking variant needs `Condvar` or `tokio::sync::Semaphore` |

## Real-World Usage
- **Hystrix** thread-pool isolation
- **Envoy** connection pool limits per upstream cluster
- **tokio::sync::Semaphore** for async Rust bulkheads
- **Resilience4j** Bulkhead module

## Further Reading
- [Release It! — Michael Nygard (Bulkhead chapter)](https://pragprog.com/titles/mnee2/)
- [Resilience4j Bulkhead docs](https://resilience4j.readme.io/docs/bulkhead)
- [Microsoft — Bulkhead Pattern](https://learn.microsoft.com/en-us/azure/architecture/patterns/bulkhead)
