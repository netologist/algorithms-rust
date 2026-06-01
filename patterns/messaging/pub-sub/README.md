# Pub-Sub (Publish-Subscribe)

## What Is This Pattern?
A **broker** decouples message producers (publishers) from consumers (subscribers).
Publishers send messages to a **topic**; all current subscribers receive a copy.
Late subscribers miss past messages (no message retention in this implementation).

## When To Use It
- Decoupled event notification (user signup → send email, update analytics)
- Fanout (one event → many handlers)
- Real-time dashboards, chat systems, notifications

## How It Works

```
Publisher ──► Broker[topic] ──► Subscriber 1
                         └───► Subscriber 2
                         └───► Subscriber 3
```

1. Subscribers call `broker.subscribe(topic)` and receive a `Receiver<String>`.
2. Publishers call `broker.publish(topic, message)`.
3. Broker delivers the message to all current subscribers of that topic.
4. No history — late subscribers only get future messages.

## Key Rust Concepts Used
- **`crossbeam_channel::unbounded`**: per-subscriber channel
- **`HashMap<String, Vec<Sender>>`**: topic → subscriber list
- **`Receiver<String>` as API**: type-safe subscription handle

## Run / Test
```bash
cargo run -p pub-sub && cargo test -p pub-sub
```

## Trade-offs
| Pro | Con |
|-----|-----|
| Complete publisher-subscriber decoupling | No message persistence (late subs miss history) |
| Easy fanout to N subscribers | Dead subscribers accumulate (need cleanup) |
| Topic isolation | No ordering guarantees across topics |

## Real-World Usage
- **Kafka**, **NATS**, **Redis Pub/Sub**
- Browser `EventEmitter`, DOM `addEventListener`
- **Google Pub/Sub**, **AWS SNS**

## Further Reading
- [Enterprise Integration Patterns — Publish-Subscribe Channel](https://www.enterpriseintegrationpatterns.com/patterns/messaging/PublishSubscribeChannel.html)
