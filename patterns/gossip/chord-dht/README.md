# Chord DHT

## What Is This Pattern?
**Chord** is a distributed hash table (DHT) that organises N nodes in a ring of
size 2^M. Each node maintains a **finger table** of O(M) shortcuts, enabling
key lookup in O(log N) hops rather than O(N).

## When To Use It
- Decentralised key-value storage
- P2P systems without central servers (BitTorrent, IPFS)
- Scalable routing in large overlay networks

## How It Works

```
M=4 ring (16 positions): 0─1─2─3─4─5─6─7─8─9─10─11─12─13─14─15─(wrap)

Nodes at: 0, 4, 8, 12
key=6 → successor(6) = 8 (node at position 8 owns key)

Finger table for node-0:
  finger[0] = successor(0 + 2^0 = 1) = 4
  finger[1] = successor(0 + 2^1 = 2) = 4
  finger[2] = successor(0 + 2^2 = 4) = 4
  finger[3] = successor(0 + 2^3 = 8) = 8
```

Lookup for key 10 starting at node-0:
  → use finger pointing closest before 10 → jump to node-8 (1 hop)
  → node-8 is responsible (owns keys 9..12) → done

## Key Rust Concepts Used
- **`BTreeMap<u64, ChordNode>`**: sorted ring with O(log N) range queries
- **`range(id..)`**: successor lookup on sorted ring
- **Finger table precomputation**: O(M) entries per node

## Run / Test
```bash
cargo run -p chord-dht && cargo test -p chord-dht
```

## Real-World Usage
- **BitTorrent DHT** (Kademlia, a Chord variant)
- **IPFS** (Kademlia DHT)
- **Ethereum** (Discovery v4/v5 based on Kademlia)

## Further Reading
- [Chord: A Scalable Peer-to-peer Lookup Service — Stoica et al. (2001)](https://pdos.csail.mit.edu/papers/chord:sigcomm01/chord_sigcomm.pdf)
