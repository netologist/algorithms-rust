use std::collections::HashMap;

pub struct Shard {
    pub name: String,
    pub min: String, // inclusive lower bound (empty = -∞)
    pub max: String, // exclusive upper bound (empty = +∞)
    store: HashMap<String, String>,
}

impl Shard {
    pub fn new(name: &str, min: &str, max: &str) -> Self {
        Self {
            name: name.into(),
            min: min.into(),
            max: max.into(),
            store: HashMap::new(),
        }
    }

    fn owns(&self, key: &str) -> bool {
        let above_min = self.min.is_empty() || key >= self.min.as_str();
        let below_max = self.max.is_empty() || key < self.max.as_str();
        above_min && below_max
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.store.get(key).map(|s| s.as_str())
    }
}

pub struct RangePartitioner {
    shards: Vec<Shard>,
}

impl RangePartitioner {
    pub fn new(shards: Vec<Shard>) -> Self {
        Self { shards }
    }

    /// Write key to its owning shard. Returns the shard name.
    pub fn write(&mut self, key: &str, value: &str) -> &str {
        let idx = self.shard_idx(key);
        self.shards[idx].set(key, value);
        &self.shards[idx].name
    }

    pub fn read(&self, key: &str) -> Option<&str> {
        self.shards.iter().find(|s| s.owns(key))?.get(key)
    }

    pub fn shard_for(&self, key: &str) -> &str {
        let idx = self.shard_idx(key);
        &self.shards[idx].name
    }

    fn shard_idx(&self, key: &str) -> usize {
        self.shards
            .iter()
            .position(|s| s.owns(key))
            .unwrap_or(self.shards.len() - 1)
    }

    pub fn shard_names(&self) -> Vec<&str> {
        self.shards.iter().map(|s| s.name.as_str()).collect()
    }
}
