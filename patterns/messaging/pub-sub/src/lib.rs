use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;

pub struct Broker {
    subscriptions: HashMap<String, Vec<Sender<String>>>,
}

impl Broker {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    /// Subscribe to a topic. Returns a receiver for incoming messages.
    pub fn subscribe(&mut self, topic: &str) -> Receiver<String> {
        let (tx, rx) = unbounded();
        self.subscriptions.entry(topic.into()).or_default().push(tx);
        rx
    }

    /// Publish a message to all current subscribers of a topic.
    /// Late subscribers (subscribed after publish) do NOT receive past messages.
    pub fn publish(&self, topic: &str, message: &str) {
        if let Some(subs) = self.subscriptions.get(topic) {
            for tx in subs {
                let _ = tx.send(message.into());
            }
        }
    }

    pub fn subscriber_count(&self, topic: &str) -> usize {
        self.subscriptions.get(topic).map_or(0, |s| s.len())
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self::new()
    }
}
