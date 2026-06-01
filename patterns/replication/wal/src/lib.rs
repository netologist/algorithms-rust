use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum EntryStatus {
    Committed,
    Uncommitted,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub sequence: u64,
    pub data: String,
    pub status: EntryStatus,
}

#[derive(Clone)]
pub struct WriteAheadLog {
    entries: Arc<Mutex<Vec<LogEntry>>>,
    next_seq: Arc<Mutex<u64>>,
}

impl WriteAheadLog {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
            next_seq: Arc::new(Mutex::new(1)),
        }
    }

    /// Append a committed entry (normal write).
    pub fn append(&self, data: &str) {
        let seq = {
            let mut n = self.next_seq.lock().unwrap();
            let s = *n;
            *n += 1;
            s
        };
        self.entries.lock().unwrap().push(LogEntry {
            sequence: seq,
            data: data.into(),
            status: EntryStatus::Committed,
        });
    }

    /// Append an uncommitted entry (simulates a crash mid-write).
    pub fn append_uncommitted(&self, data: &str) {
        let seq = {
            let mut n = self.next_seq.lock().unwrap();
            let s = *n;
            *n += 1;
            s
        };
        self.entries.lock().unwrap().push(LogEntry {
            sequence: seq,
            data: data.into(),
            status: EntryStatus::Uncommitted,
        });
    }

    /// Return only committed entries.
    pub fn committed_entries(&self) -> Vec<LogEntry> {
        self.entries
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.status == EntryStatus::Committed)
            .cloned()
            .collect()
    }

    pub fn all_entries(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }
}

impl Default for WriteAheadLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple key-value store that recovers state from a WAL.
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Replay all committed SET operations from the log.
    pub fn recover_from(&mut self, log: &WriteAheadLog) {
        for entry in log.committed_entries() {
            self.apply(&entry.data);
        }
    }

    /// Apply a single "SET key value" operation.
    fn apply(&mut self, op: &str) {
        let parts: Vec<&str> = op.splitn(3, ' ').collect();
        if parts.len() == 3 && parts[0] == "SET" {
            self.data.insert(parts[1].into(), parts[2].into());
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
