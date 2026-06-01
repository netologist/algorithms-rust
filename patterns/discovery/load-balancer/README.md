# Load Balancer

## What Is This Pattern?
A **load balancer** distributes requests across multiple backends according to
a routing algorithm. This implements three classic algorithms:
Round-Robin, Least-Connections, and Consistent-Hash.

## When To Use It
- Distributing HTTP traffic across a backend pool
- Stateful session routing (consistent-hash by session ID)
- Avoiding overloading a single backend

## Algorithms

| Algorithm | How it works | Best for |
|-----------|-------------|----------|
| Round-Robin | Cycle through backends in order | Stateless, equal-capacity backends |
| Least-Connections | Pick the backend with fewest active connections | Varying request duration |
| Consistent-Hash | hash(key) % N — same key → same backend | Session affinity |

## Key Rust Concepts Used
- **`%` cursor**: round-robin via `rr_index % len`
- **`min_by_key`**: least-connections selection
- **`DefaultHasher`**: stable consistent-hash mapping
- **`saturating_sub`**: prevents underflow on disconnect

## Run / Test
```bash
cargo run -p load-balancer && cargo test -p load-balancer
```

## Real-World Usage
- **Nginx**, **HAProxy**, **Envoy** (all three algorithms)
- **AWS ALB** (round-robin + least-outstanding-requests)
- **Kubernetes Service** (iptables round-robin)

## Further Reading
- [Load Balancing — Wikipedia](https://en.wikipedia.org/wiki/Load_balancing_(computing))
