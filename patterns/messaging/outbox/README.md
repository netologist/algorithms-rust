# Transactional Outbox

## What Is This Pattern?
Instead of writing to the database AND the message broker in two separate
operations (which can fail independently), write to the DB and an **outbox
table** in a **single transaction**. A background **relay** reads the outbox
and publishes pending entries to the broker, then marks them published.

## When To Use It
- Guaranteeing at-least-once event delivery
- Avoiding the dual-write problem (DB + broker in the same "transaction")
- Microservices that must reliably emit events on state changes

## How It Works

```
Transaction:
  INSERT INTO orders VALUES (...)   ← business write
  INSERT INTO outbox VALUES (...)   ← outbox entry
  COMMIT

Background relay:
  SELECT * FROM outbox WHERE published = false
  → broker.publish(topic, payload)
  UPDATE outbox SET published = true WHERE id = ?
```

If the process crashes before the relay runs, the outbox entry survives and
will be published on the next poll. **At-least-once** delivery: the relay
may publish duplicates; consumers must be idempotent.

## Key Rust Concepts Used
- **Simulated atomic write**: `write_with_outbox` modifies both `records` and `outbox` together
- **`relay` function**: separates relay concern from DB
- **`filter(!e.published)`**: idempotent relay check

## Run / Test
```bash
cargo run -p outbox && cargo test -p outbox
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Guaranteed at-least-once delivery | Relay adds operational complexity |
| No dual-write atomicity problem | Messages may be delivered more than once |
| Works with any message broker | Outbox table can grow (needs cleanup) |

## Real-World Usage
- **Debezium** (CDC-based outbox relay)
- **Transactional outbox** in most microservices frameworks
- **Eventuate Tram** (Spring-based implementation)

## Further Reading
- [Chris Richardson — Transactional Outbox](https://microservices.io/patterns/data/transactional-outbox.html)
