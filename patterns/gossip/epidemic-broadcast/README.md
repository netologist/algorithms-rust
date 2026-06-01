# Epidemic Broadcast (SI Model)

## What Is This Pattern?
**Epidemic broadcast** models information spread as a disease using the
**SI (Susceptible-Infected)** model. Once a node is infected (has the data),
it actively tries to infect Susceptible neighbours. Dead nodes do not
block convergence.

## When To Use It
- Reliable broadcast without a central server
- Fault-tolerant data dissemination
- Content distribution in P2P networks

## How It Works

```
SI Model:
  S (Susceptible) → I (Infected) → [stays infected forever]

Infected nodes gossip to random peers each round.
Dead nodes are skipped — remaining nodes still converge.
```

Differs from basic gossip in that infection is one-directional and
the data is a single broadcast payload.

## Key Rust Concepts Used
- **`InfectionState` enum**: clean state machine
- **Alive guard**: dead nodes skip gossip rounds
- **`or_insert_with`**: idempotent infection

## Run / Test
```bash
cargo run -p epidemic-broadcast && cargo test -p epidemic-broadcast
```

## Real-World Usage
- **BitTorrent** (piece propagation)
- **IPFS** (content routing)
- **Waku** (Ethereum P2P messaging)

## Further Reading
- [Epidemic Algorithms — Demers et al.](https://dl.acm.org/doi/10.1145/41840.41841)
