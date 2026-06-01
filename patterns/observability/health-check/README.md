# Health Check

## What Is This Pattern?
A **health check endpoint** lets a load balancer or orchestrator verify that
a service is ready to receive traffic. Multiple internal checks (DB, cache,
disk) are aggregated into a single overall status: Healthy / Degraded / Unhealthy.

## When To Use It
- Kubernetes liveness/readiness probes
- Load balancer backend health checks
- Alerting and on-call routing

## How It Works

```
GET /health

{
  "status": "Degraded",
  "checks": {
    "database":   "Healthy",
    "cache":      "Unhealthy",
    "disk-space": "Healthy"
  }
}
```

**Aggregation rule:**
- All pass → Healthy
- Some pass → Degraded
- None pass → Unhealthy

## Key Rust Concepts Used
- **`Box<dyn Fn() -> HealthStatus>`**: pluggable check implementations
- **Builder pattern**: `.add_check(...)` for composable endpoint
- **`filter` + count**: aggregation without mutation

## Run / Test
```bash
cargo run -p health-check && cargo test -p health-check
```

## Real-World Usage
- **Kubernetes** liveness/readiness/startup probes
- **AWS ALB** target group health checks
- **Consul** service health checks

## Further Reading
- [Health Check API Pattern — microservices.io](https://microservices.io/patterns/observability/health-check-api.html)
