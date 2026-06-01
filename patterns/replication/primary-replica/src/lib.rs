use crossbeam_channel::{unbounded, Sender};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
enum ReplicationMsg {
    Set { key: String, value: String },
}

#[derive(Clone)]
pub struct Store(Arc<Mutex<HashMap<String, String>>>);

impl Store {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    fn set(&self, key: &str, value: &str) {
        self.0.lock().unwrap().insert(key.into(), value.into());
    }

    fn get(&self, key: &str) -> Option<String> {
        self.0.lock().unwrap().get(key).cloned()
    }
}

pub struct ReplicationCluster {
    primary: Store,
    replicas: Vec<Store>,
    tx: Sender<ReplicationMsg>,
}

impl ReplicationCluster {
    pub fn new(replica_count: usize) -> Self {
        let primary = Store::new();
        let replicas: Vec<Store> = (0..replica_count).map(|_| Store::new()).collect();
        let (tx, rx) = unbounded::<ReplicationMsg>();

        // Spawn replication worker
        let replica_clones: Vec<Store> = replicas.clone();
        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    ReplicationMsg::Set { key, value } => {
                        for replica in &replica_clones {
                            replica.set(&key, &value);
                        }
                    }
                }
            }
        });

        Self {
            primary,
            replicas,
            tx,
        }
    }

    /// Write to primary and asynchronously replicate to all replicas.
    pub fn write(&self, key: &str, value: &str) {
        self.primary.set(key, value);
        let _ = self.tx.send(ReplicationMsg::Set {
            key: key.into(),
            value: value.into(),
        });
    }

    pub fn read_from_primary(&self, key: &str) -> Option<String> {
        self.primary.get(key)
    }

    pub fn read_from_replica(&self, idx: usize, key: &str) -> Option<String> {
        self.replicas[idx].get(key)
    }

    pub fn replica_count(&self) -> usize {
        self.replicas.len()
    }
}
