# Circuit Breaker

## What Is This Pattern?
The Circuit Breaker prevents an application from repeatedly calling a service that
is failing. It wraps calls in a state machine: **Closed** (normal), **Open** (reject
all calls), and **HalfOpen** (probe with one call). After a configurable number of
failures the circuit opens; after a timeout it allows a probe call; after enough
successes it closes again.

## When To Use It
- Calling external HTTP APIs that may go down
- Database connection pools under saturation
- Any I/O call where repeated failure wastes resources or triggers cascading failures

## How It Works

```
         failures >= threshold
Closed ─────────────────────────► Open
  ▲                                 │
  │ successes >= threshold          │ timeout elapsed
  │                                 ▼
  └──────────────────────────── HalfOpen
```

1. **Closed**: every call passes through; failures increment a counter.
2. **Open**: every call is rejected immediately without touching the service.
3. **HalfOpen**: after `half_open_timeout`, one probe call is allowed.
   - Success → increment success counter; once threshold reached → close
   - Failure → reopen (reset timeout)

## This Implementation
- `CircuitBreaker` struct holds state, counters, and last-failure timestamp.
- `call<T, E, F>()` is generic — works with any fallible function.
- `state()` lazily transitions Open→HalfOpen when the timeout has elapsed.
- Demo: 3 failures → open → wait 300ms → half-open → 2 successes → closed.

## Key Rust Concepts Used
- **Enum state machine**: `CircuitState` with `PartialEq` for state checks
- **`Instant::elapsed()`**: timeout tracking without async
- **Generic `call<T,E,F>`**: wraps any `FnOnce() -> Result<T,E>`
- **`CircuitError<E>`**: enum that distinguishes open-circuit from inner errors

## Run
```bash
cargo run -p circuit-breaker
```

## Test
```bash
cargo test -p circuit-breaker
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Prevents cascading failures | Threshold tuning is application-specific |
| Fast-fail reduces latency during outages | False positives can reject valid traffic |
| Self-healing via HalfOpen | State is per-instance (not distributed) |

## Real-World Usage
- **Netflix Hystrix** / **Resilience4j** (Java)
- **Envoy proxy** (built-in outlier detection)
- **Polly** (.NET)
- `failsafe-rs` crate

## Further Reading
- [Martin Fowler — CircuitBreaker](https://martinfowler.com/bliki/CircuitBreaker.html)
- [Release It! — Michael Nygard](https://pragprog.com/titles/mnee2/release-it-second-edition/)
- [AWS — Implementing the Circuit Breaker Pattern](https://aws.amazon.com/builders-library/avoiding-overload-in-distributed-systems-by-putting-the-smaller-service-in-control/)
