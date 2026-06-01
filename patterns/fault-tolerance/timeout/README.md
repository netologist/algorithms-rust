# Timeout

## What Is This Pattern?
A **timeout** places a deadline on an operation. If the operation doesn't complete
within the deadline, the caller receives an error immediately rather than blocking
indefinitely. This prevents slow upstream dependencies from propagating latency
throughout a system.

## When To Use It
- Any network call (HTTP, gRPC, database query)
- Acquiring locks or resources that may be contended
- Long-running background computations with SLA requirements
- Anywhere an unbounded wait could cascade into a full system freeze

## How It Works

```
Caller                  Background Thread
  │                           │
  ├── spawn(f) ──────────────►│
  │                           │ (working...)
  │◄─ recv_timeout(deadline) ─┤
  │                           │
  │  if result arrives:       │
  │    → Ok(value)            │
  │  if deadline elapses:     │
  │    → Err(Elapsed)         │
  │  (thread keeps running)   │
```

1. `f` is spawned in a background thread.
2. The caller waits on a bounded channel with a `recv_timeout`.
3. If the result arrives in time → `Ok(T)`.
4. If the deadline elapses → `Err(TimeoutError::Elapsed)`.

**Important:** the background thread cannot be killed — it runs to completion.
Production patterns use cancellation tokens (`CancellationToken` from `tokio-util`)
or `tokio::time::timeout` with cooperative cancellation.

## This Implementation
- `with_timeout<T, F>(deadline, f)` — generic over return type and closure
- `crossbeam_channel::bounded(1)` — the channel used for the race
- `recv_timeout` — blocks for at most `deadline`

## Key Rust Concepts Used
- **`Send + 'static` bounds**: required to move closure into a spawned thread
- **`crossbeam_channel::bounded(1)`**: single-slot rendezvous channel
- **`recv_timeout`**: blocking receive with a deadline

## Run
```bash
cargo run -p timeout
```

## Test
```bash
cargo test -p timeout
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Prevents indefinite blocking | Background thread still runs after timeout |
| Works without async | Not composable with `async/await` directly |
| Simple, zero external I/O dependencies | Cannot interrupt CPU-bound work |

## Real-World Usage
- **tokio::time::timeout** — async equivalent with cancellation
- **reqwest** — `.timeout(Duration)` on HTTP requests
- **PostgreSQL** — `statement_timeout` config
- **gRPC** — deadline propagation across service calls

## Further Reading
- [tokio::time::timeout docs](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html)
- [Google SRE Book — Handling Overload](https://sre.google/sre-book/handling-overload/)
