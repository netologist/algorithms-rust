# Sidecar

## What Is This Pattern?
A **sidecar** is a helper process (or thread) that runs alongside the main
application and handles cross-cutting concerns: auth, logging, retry, circuit
breaking, metrics, mTLS. The main app calls through the sidecar for all outbound
requests.

## When To Use It
- Adding observability without changing application code
- Centralising retry/circuit-breaker logic
- Language-agnostic service mesh (Envoy, Linkerd)

## How It Works

```
Main App  ──outbound──►  Sidecar  ──► External Service
                           │
                           ├── Add auth header
                           ├── Log request/response
                           ├── Record metrics
                           └── Apply circuit breaker
```

## Key Rust Concepts Used
- **`Box<dyn Interceptor>`**: dynamic dispatch for pluggable middleware
- **Builder pattern**: `.add_interceptor(...)` chains
- **`Send + Sync` trait bounds**: interceptors are safe across threads

## Run / Test
```bash
cargo run -p sidecar && cargo test -p sidecar
```

## Real-World Usage
- **Envoy** proxy (Istio service mesh)
- **Dapr** sidecar
- **Linkerd** data plane

## Further Reading
- [Sidecar Pattern — Microsoft Docs](https://learn.microsoft.com/en-us/azure/architecture/patterns/sidecar)
- [Designing Distributed Systems — Brendan Burns](https://www.oreilly.com/library/view/designing-distributed-systems/9781491983638/)
