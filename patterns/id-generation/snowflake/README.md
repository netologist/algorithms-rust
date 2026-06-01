# Snowflake ID Generation

A distributed **unique ID generation** algorithm that produces time-ordered, 64-bit IDs without central coordination.

## ID Layout (Twitter Snowflake)

```
 63      62                     22   21     17   16    12   11          0
  0 │      41-bit timestamp (ms)   │  5-bit DC  │ 5-bit M  │ 12-bit seq
sign │  ms since custom epoch 2020  │ datacenter │ machine  │  counter
```

| Field | Bits | Range | Notes |
|-------|------|-------|-------|
| Sign | 1 | always 0 | keeps ID positive |
| Timestamp | 41 | ~69 years from epoch | ms since 2020-01-01 |
| Datacenter | 5 | 0–31 | logical datacenter |
| Machine | 5 | 0–31 | node within datacenter |
| Sequence | 12 | 0–4095 | counter resets each ms |

**Max throughput**: 4 096 IDs/ms per (datacenter, machine) = **4M IDs/s per node**.

## Properties

* **Time-ordered** — lexicographic order ≈ insertion order (useful for DB indexes).
* **Decentralised** — no coordination required between generators.
* **Parseable** — timestamp, datacenter, machine, and sequence are recoverable.
* **Compact** — fits in a `u64` / `BIGINT`.

## Use Cases

* Primary keys for distributed databases (avoid UUID fragmentation).
* Event IDs in event-sourcing / message queues.
* Sharding keys (timestamp portion enables time-range queries).
* Correlation IDs in distributed tracing.

## Running

```bash
cargo run   -p snowflake
cargo test  -p snowflake
```
