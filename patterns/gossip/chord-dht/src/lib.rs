use std::collections::BTreeMap;
use std::collections::HashMap;

/// Chord DHT with M-bit ring (ring size = 2^M).
///
/// Each node stores:
/// - `successor`: next node clockwise on the ring
/// - `finger_table[i]`: successor(id + 2^i mod 2^M)
/// - `store`: keys this node is responsible for

pub struct ChordNode {
    pub id: u64,
    pub successor: u64,
    pub finger_table: Vec<u64>,
    pub store: HashMap<u64, String>,
}

pub struct ChordRing {
    pub m: u8,
    pub nodes: BTreeMap<u64, ChordNode>,
}

impl ChordRing {
    pub fn new(m: u8) -> Self {
        Self {
            m,
            nodes: BTreeMap::new(),
        }
    }

    fn ring_size(&self) -> u64 {
        1u64 << self.m
    }

    fn successor_of(&self, id: u64) -> u64 {
        let id = id % self.ring_size();
        self.nodes
            .range(id..)
            .next()
            .or_else(|| self.nodes.iter().next())
            .map(|(&k, _)| k)
            .unwrap_or(id)
    }

    pub fn join(&mut self, node_id: u64) {
        let id = node_id % self.ring_size();
        let succ = if self.nodes.is_empty() {
            id
        } else {
            self.successor_of(id + 1)
        };
        self.nodes.insert(
            id,
            ChordNode {
                id,
                successor: succ,
                finger_table: vec![],
                store: HashMap::new(),
            },
        );
    }

    pub fn build_finger_tables(&mut self) {
        let ids: Vec<u64> = self.nodes.keys().cloned().collect();
        for &id in &ids {
            let fingers: Vec<u64> = (0..self.m)
                .map(|i| self.successor_of(id.wrapping_add(1u64 << i)))
                .collect();
            self.nodes.get_mut(&id).unwrap().finger_table = fingers;
        }
    }

    /// Look up a key. Returns (responsible_node_id, hop_count).
    pub fn lookup(&self, key: u64) -> (u64, usize) {
        let key = key % self.ring_size();
        // Start at first node
        let start = *self.nodes.keys().next().unwrap();
        let mut current = start;
        let mut hops = 0;

        for _ in 0..(self.m as usize + 1) {
            let node = &self.nodes[&current];
            if self.is_responsible(current, key) {
                return (current, hops);
            }
            // Forward to closest finger that precedes the key
            let next = node
                .finger_table
                .iter()
                .rev()
                .find(|&&f| self.precedes(current, f, key))
                .copied()
                .unwrap_or(node.successor);
            if next == current {
                break;
            }
            current = next;
            hops += 1;
        }
        (current, hops)
    }

    /// Check if `node_id` is responsible for `key`.
    fn is_responsible(&self, node_id: u64, key: u64) -> bool {
        let pred = self.predecessor_of(node_id);
        if pred < node_id {
            key > pred && key <= node_id
        } else {
            key > pred || key <= node_id // wrap-around
        }
    }

    fn predecessor_of(&self, id: u64) -> u64 {
        self.nodes
            .range(..id)
            .next_back()
            .or_else(|| self.nodes.iter().next_back())
            .map(|(&k, _)| k)
            .unwrap_or(id)
    }

    /// Return true if `f` is between `current` (exclusive) and `key` (exclusive).
    fn precedes(&self, current: u64, f: u64, key: u64) -> bool {
        if current < key {
            f > current && f < key
        } else {
            f > current || f < key
        } // wrap
    }

    pub fn put(&mut self, key: u64, value: &str) {
        let key = key % self.ring_size();
        let owner = self.successor_of(key);
        self.nodes
            .get_mut(&owner)
            .unwrap()
            .store
            .insert(key, value.into());
    }

    pub fn get(&self, key: u64) -> Option<&str> {
        let key = key % self.ring_size();
        let owner = self.successor_of(key);
        self.nodes[&owner].store.get(&key).map(|s| s.as_str())
    }
}
