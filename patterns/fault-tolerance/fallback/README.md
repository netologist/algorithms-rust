# Fallback

## What Is This Pattern?
A **fallback** provides an alternative response when the primary call fails. Instead
of propagating an error to the user, the system returns a degraded-but-useful value:
a cached response, a default, a simpler computation, or a static placeholder.

## When To Use It
- API calls that can return stale-but-acceptable cached data on failure
- Feature flags: primary = new feature, fallback = old behaviour
- Payment gateway: primary = Stripe, fallback = PayPal
- Search: primary = semantic search, fallback = keyword search
- Any situation where partial availability beats total failure

## How It Works

```
Request
   │
   ▼
Primary call ──► success? ──► return result
   │
   └──► failure ──► Fallback ──► cached / default / secondary
```

Two variants in this implementation:

1. **`with_fallback(primary, fallback)`** — stateless; call primary, call fallback on error.
2. **`FallbackCache<T>`** — stateful; remembers the last successful response and
   serves it on failure (stale-while-revalidate pattern).

## This Implementation
- `with_fallback<T,E,P,F>` — generic function, zero allocations
- `FallbackCache<T>` — `Option<T>` internal; clones on cache hit
- Cache is updated only on success, so stale data doesn't get stalier

## Key Rust Concepts Used
- **`FnOnce` closures**: primary and fallback are called at most once
- **`unwrap_or_else`**: idiomatic fallback on `Result`
- **Generic `T: Clone`**: cache stores owned values that can be returned by value

## Run
```bash
cargo run -p fallback
```

## Test
```bash
cargo test -p fallback
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Users see degraded service instead of errors | Stale data may be incorrect or misleading |
| Zero latency overhead on success path | Fallback logic must be maintained separately |
| Composable with circuit breaker + retry | Cache invalidation logic adds complexity |

## Real-World Usage
- **Netflix** — show cached content metadata when recommendation service is down
- **CDN edge caches** — serve stale content on origin failure
- **Feature flags** — `LaunchDarkly` SDK falls back to defaults on SDK failure

## Further Reading
- [Microsoft — Fallback Pattern](https://learn.microsoft.com/en-us/azure/architecture/patterns/fallback)
- [Resilience4j Fallback](https://resilience4j.readme.io/docs/fallback)
