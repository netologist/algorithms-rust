# Distributed Tracing

## What Is This Pattern?
**Distributed tracing** tracks a request as it flows through multiple services.
A **TraceContext** (trace_id + span_id) is propagated in request headers.
Each service creates a **Span** (start time, duration, parent span) and reports
it to a trace collector. The result is a tree of spans showing the full request journey.

## When To Use It
- Debugging latency in multi-service requests
- Understanding service dependencies
- Finding which service introduced an error

## How It Works

```
Client → [api-gateway] ──────────────────────────────── Span A (root)
              └──► [order-service] ─────────────────── Span B (parent=A)
                        ├──► [inventory-service] ─── Span C (parent=B)
                        └──► [payment-service]   ─── Span D (parent=B)
```

Trace context (trace_id + parent_span_id) is passed between services.
All spans share the same trace_id.

## Key Rust Concepts Used
- **`Instant` for timing**: `start.elapsed().as_micros()` for span duration
- **`Arc<Mutex<HashMap>>`**: thread-safe span store
- **`Option<SpanId>`**: root spans have no parent
- **Recursive indent**: `print_trace` computes depth by following parent chain

## Run / Test
```bash
cargo run -p distributed-tracing && cargo test -p distributed-tracing
```

## Real-World Usage
- **Jaeger**, **Zipkin** (open-source trace collectors)
- **OpenTelemetry** (OTLP standard)
- **AWS X-Ray**, **Google Cloud Trace**, **Datadog APM**

## Further Reading
- [OpenTelemetry Spec](https://opentelemetry.io/docs/concepts/signals/traces/)
- [Dapper — Google's Distributed Tracing System (2010)](https://research.google/pubs/dapper-a-large-scale-distributed-systems-tracing-infrastructure/)
