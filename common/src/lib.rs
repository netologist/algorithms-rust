//! Shared simulation primitives for distributed system pattern demos.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub use crossbeam_channel::{unbounded, Receiver, Sender};
use rand::Rng;

/// Unique node identifier.
pub type NodeId = u64;

/// Lamport logical clock.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicalClock(pub u64);

impl LogicalClock {
    /// Increment before sending a message.
    pub fn tick(&mut self) -> Self {
        self.0 += 1;
        *self
    }

    /// Update on receive: max(local, received) + 1.
    pub fn update(&mut self, received: LogicalClock) {
        self.0 = self.0.max(received.0) + 1;
    }
}

/// Generic message envelope. Payload is serde_json-encoded by the sender.
#[derive(Debug, Clone)]
pub struct Message {
    pub from: NodeId,
    pub to: NodeId,
    pub payload: Vec<u8>,
    pub timestamp: LogicalClock,
}

impl Message {
    pub fn new<T: serde::Serialize>(
        from: NodeId,
        to: NodeId,
        ts: LogicalClock,
        payload: &T,
    ) -> Self {
        Self {
            from,
            to,
            payload: serde_json::to_vec(payload).expect("serialize"),
            timestamp: ts,
        }
    }

    pub fn decode<T: serde::de::DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.payload).expect("deserialize")
    }
}

/// A simulated network node with an inbox channel.
pub struct SimNode {
    pub id: NodeId,
    pub tx: Sender<Message>,
    pub rx: Receiver<Message>,
}

impl SimNode {
    pub fn new(id: NodeId) -> Self {
        let (tx, rx) = unbounded();
        Self { id, tx, rx }
    }
}

/// In-memory network bus.
///
/// - `latency_ms`: sleep before delivering each message (0 = instant)
/// - `drop_rate`: probability (0.0–1.0) that a message is silently dropped
pub struct NetworkSim {
    nodes: HashMap<NodeId, Sender<Message>>,
    latency_ms: u64,
    drop_rate: f64,
}

impl Default for NetworkSim {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkSim {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            latency_ms: 0,
            drop_rate: 0.0,
        }
    }

    pub fn with_latency(mut self, ms: u64) -> Self {
        self.latency_ms = ms;
        self
    }

    pub fn with_drop_rate(mut self, rate: f64) -> Self {
        self.drop_rate = rate.clamp(0.0, 1.0);
        self
    }

    pub fn register(&mut self, node: &SimNode) {
        self.nodes.insert(node.id, node.tx.clone());
    }

    pub fn send(&self, msg: Message) {
        if let Some(tx) = self.nodes.get(&msg.to) {
            if self.should_drop() {
                return;
            }
            let tx = tx.clone();
            let latency = self.latency_ms;
            thread::spawn(move || {
                if latency > 0 {
                    thread::sleep(Duration::from_millis(latency));
                }
                let _ = tx.send(msg);
            });
        }
    }

    pub fn broadcast(&self, from: NodeId, payload: Vec<u8>, clock: LogicalClock) {
        for (&to, tx) in &self.nodes {
            if to == from {
                continue;
            }
            let msg = Message {
                from,
                to,
                payload: payload.clone(),
                timestamp: clock,
            };
            if self.should_drop() {
                continue;
            }
            let tx = tx.clone();
            let latency = self.latency_ms;
            thread::spawn(move || {
                if latency > 0 {
                    thread::sleep(Duration::from_millis(latency));
                }
                let _ = tx.send(msg);
            });
        }
    }

    fn should_drop(&self) -> bool {
        if self.drop_rate == 0.0 {
            return false;
        }
        rand::thread_rng().gen::<f64>() < self.drop_rate
    }
}

/// Generate a random NodeId.
pub fn random_id() -> NodeId {
    rand::thread_rng().gen()
}

/// Sleep for `ms` milliseconds.
pub fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_clock_tick_increments() {
        let mut c = LogicalClock::default();
        c.tick();
        c.tick();
        assert_eq!(c.0, 2);
    }

    #[test]
    fn logical_clock_update_takes_max_plus_one() {
        let mut local = LogicalClock(5);
        local.update(LogicalClock(10));
        assert_eq!(local.0, 11);

        let mut local2 = LogicalClock(10);
        local2.update(LogicalClock(5));
        assert_eq!(local2.0, 11);
    }

    #[test]
    fn network_sim_delivers_message() {
        let node_a = SimNode::new(1);
        let node_b = SimNode::new(2);

        let mut net = NetworkSim::new();
        net.register(&node_a);
        net.register(&node_b);

        net.send(Message {
            from: 1,
            to: 2,
            payload: b"hello".to_vec(),
            timestamp: LogicalClock(1),
        });

        let msg = node_b.rx.recv_timeout(Duration::from_millis(100)).unwrap();
        assert_eq!(msg.payload, b"hello");
    }

    #[test]
    fn message_encode_decode_roundtrip() {
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
        struct Ping {
            seq: u32,
        }
        let msg = Message::new(1, 2, LogicalClock(1), &Ping { seq: 42 });
        let decoded: Ping = msg.decode();
        assert_eq!(decoded.seq, 42);
    }
}
