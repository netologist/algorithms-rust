# Fault Tolerance Patterns

Patterns that keep a system available and responsive when parts of it fail,
slow down, or become overloaded.

## Patterns

| Pattern | Crate | Description |
|---------|-------|-------------|
| [Circuit Breaker](circuit-breaker/) | `circuit-breaker` | State machine (Closed → Open → HalfOpen) that fast-fails calls to broken dependencies |
| [Retry with Backoff](retry-backoff/) | `retry-backoff` | Exponential backoff with jitter — retry failed operations with increasing delays |
| [Bulkhead](bulkhead/) | `bulkhead` | Bounded concurrency per resource — isolates failures to a subset of threads/connections |
| [Timeout](timeout/) | `timeout` | Enforce maximum wait time; fail fast instead of hanging indefinitely |
| [Fallback](fallback/) | `fallback` | Return a degraded/cached/static response when the primary path fails |

## Key Rust Concepts

- Enum state machines (`CircuitState`) with `PartialEq`
- Generic function wrappers: `call<T, E, F>(fn) -> Result<T, E>`
- `std::thread::spawn` + `crossbeam::channel` for bounded parallelism
- `Instant::elapsed()` for timeout tracking (no async required)
