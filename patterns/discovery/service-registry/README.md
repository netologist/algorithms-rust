# Service Registry

## What Is This Pattern?
A **service registry** is a database of available service instances. Services
register themselves on startup with their address and a TTL. Clients discover
services by querying the registry by name. Stale entries expire automatically.

## When To Use It
- Microservices with dynamic addresses (containers, VMs)
- Service discovery before load balancing
- Health-based routing (only return healthy instances)

## How It Works

```
Service A starts → register("api", addr, ttl=60s) → Registry
Client           → lookup("api")                  → Registry → [addr1, addr2]
TTL expires      → entry evicted on next lookup
```

## Key Rust Concepts Used
- **`Instant::elapsed()`**: TTL check without external timers
- **`HashMap<String, Vec<ServiceInstance>>`**: multi-instance per service name
- **Lazy eviction**: stale entries removed on `lookup`, not on a timer

## Run / Test
```bash
cargo run -p service-registry && cargo test -p service-registry
```

## Real-World Usage
- **Consul**, **etcd**, **ZooKeeper** (server-side registry)
- **Eureka** (Netflix), **Nacos** (Alibaba)
- **Kubernetes** Service + DNS

## Further Reading
- [Service Discovery in Microservices — Chris Richardson](https://microservices.io/patterns/service-registry.html)
