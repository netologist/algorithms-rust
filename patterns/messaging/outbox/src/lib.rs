/// Transactional Outbox Pattern.
///
/// A write to the business table and to the outbox table happens atomically.
/// A background relay reads the outbox and publishes pending entries to the broker,
/// then marks them as published.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OutboxEntry {
    pub id: u64,
    pub topic: String,
    pub payload: String,
    pub published: bool,
}

pub struct Database {
    pub records: HashMap<String, String>,
    pub outbox: Vec<OutboxEntry>,
    next_id: u64,
}

impl Database {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            outbox: vec![],
            next_id: 1,
        }
    }

    /// Atomically write a business record AND an outbox entry.
    /// In production this would be a single DB transaction.
    pub fn write_with_outbox(&mut self, key: &str, value: &str, topic: &str, payload: &str) {
        self.records.insert(key.into(), value.into());
        let id = self.next_id;
        self.next_id += 1;
        self.outbox.push(OutboxEntry {
            id,
            topic: topic.into(),
            payload: payload.into(),
            published: false,
        });
    }

    pub fn pending_outbox(&self) -> Vec<&OutboxEntry> {
        self.outbox.iter().filter(|e| !e.published).collect()
    }

    pub fn mark_published(&mut self, id: u64) {
        if let Some(entry) = self.outbox.iter_mut().find(|e| e.id == id) {
            entry.published = true;
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InMemoryBroker {
    pub published: Vec<(String, String)>, // (topic, payload)
}

impl InMemoryBroker {
    pub fn new() -> Self {
        Self { published: vec![] }
    }

    pub fn publish(&mut self, topic: &str, payload: &str) {
        self.published.push((topic.into(), payload.into()));
    }
}

impl Default for InMemoryBroker {
    fn default() -> Self {
        Self::new()
    }
}

/// Relay: polls pending outbox entries and publishes them.
pub fn relay(db: &mut Database, broker: &mut InMemoryBroker) {
    let pending: Vec<(u64, String, String)> = db
        .pending_outbox()
        .iter()
        .map(|e| (e.id, e.topic.clone(), e.payload.clone()))
        .collect();

    for (id, topic, payload) in pending {
        broker.publish(&topic, &payload);
        db.mark_published(id);
        println!(
            "  [relay] Published outbox-{}: topic='{}' payload='{}'",
            id, topic, payload
        );
    }
}
