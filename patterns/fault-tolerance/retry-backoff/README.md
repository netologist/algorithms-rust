# Retry with Exponential Backoff

## What Is This Pattern?
When a transient failure occurs (network hiccup, temporary overload), immediately
retrying often hits the same error. **Exponential backoff** waits progressively
longer between retries (delay doubles each attempt). **Jitter** adds randomness to
avoid the *thundering herd* problem where many clients retry simultaneously and
re-overload the recovering service.

## When To Use It
- HTTP requests to external APIs
- Database reconnection after a dropped connection
- Message queue consumers facing temporary broker unavailability
- Any idempotent operation that may transiently fail

## How It Works

```
Call 1 → FAIL
   sleep(base × 2⁰ ± jitter)   →  ~100ms
Call 2 → FAIL
   sleep(base × 2¹ ± jitter)   →  ~200ms
Call 3 → FAIL
   sleep(base × 2² ± jitter)   →  ~400ms
Call 4 → SUCCESS ✓
```

**Formula:** `delay(n) = min(base × 2ⁿ, max_delay) × (1 ± 25%)`

Jitter spreads retries across time, preventing multiple clients from hitting
the server at exactly the same moment.

## This Implementation
- `retry(config, f)` — generic over any `FnMut() -> Result<T, E>`
- `RetryConfig::delay_for(n)` — computes per-attempt delay without sleeping
- Jitter is ±25% of the computed capped delay

## Key Rust Concepts Used
- **`FnMut` closure**: allows mutable captures for stateful retry lambdas
- **`std::thread::sleep`**: blocking delay, no async needed
- **`rand::thread_rng().gen_range`**: per-attempt jitter
- **Generic error type `E`**: works with any error, no boxing required

## Run
```bash
cargo run -p retry-backoff
```

## Test
```bash
cargo test -p retry-backoff
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Handles transient failures transparently | Adds latency when failures persist |
| Jitter prevents thundering herd | Wrong for non-idempotent operations |
| Zero dependencies | Needs Circuit Breaker to avoid infinite retry loops |
| Composable with other patterns | Max retries must be tuned per use-case |

## Real-World Usage
- **AWS SDK** default retry policy (exponential backoff + jitter)
- **Google Cloud client libraries**
- `reqwest-retry` / `tower::retry` Rust crates
- **gRPC** deadline propagation pairs with this

## Further Reading
- [AWS: Exponential Backoff and Jitter](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/)
- [Google Cloud: Retry Strategy](https://cloud.google.com/storage/docs/retry-strategy)
- [Marc Brooker: Jitter Paper](https://brooker.co.za/blog/2022/02/28/retries.html)
