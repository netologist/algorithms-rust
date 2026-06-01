use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A space-efficient probabilistic data structure for set membership tests.
///
/// A Bloom filter can tell you with certainty that an element is NOT in the set,
/// but may produce false positives (saying an element IS in the set when it isn't).
///
/// # Parameters
/// - `size`: number of bits in the underlying bit vector
/// - `num_hashes`: number of independent hash functions to apply per element
///
/// # Trade-offs
/// - Higher `size` → lower false-positive rate, more memory
/// - Higher `num_hashes` → lower false-positive rate up to an optimal point,
///   then increases again
///
/// Optimal `num_hashes = (size / n) * ln(2)` where `n` is expected element count.
pub struct BloomFilter {
    bit_vec: Vec<bool>,
    num_hashes: usize,
    count: usize,
}

impl BloomFilter {
    /// Create a new Bloom filter with `size` bits and `num_hashes` hash functions.
    pub fn new(size: usize, num_hashes: usize) -> Self {
        assert!(size > 0, "size must be > 0");
        assert!(num_hashes > 0, "num_hashes must be > 0");
        BloomFilter {
            bit_vec: vec![false; size],
            num_hashes,
            count: 0,
        }
    }

    fn hash<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(seed);
        item.hash(&mut hasher);
        (hasher.finish() as usize) % self.bit_vec.len()
    }

    /// Insert an element into the filter.
    pub fn insert<T: Hash>(&mut self, item: &T) {
        for seed in 0..self.num_hashes as u64 {
            let pos = self.hash(item, seed);
            self.bit_vec[pos] = true;
        }
        self.count += 1;
    }

    /// Return `false` if the element is definitely NOT in the set.
    /// Return `true` if the element is probably in the set (may be a false positive).
    pub fn may_contain<T: Hash>(&self, item: &T) -> bool {
        for seed in 0..self.num_hashes as u64 {
            let pos = self.hash(item, seed);
            if !self.bit_vec[pos] {
                return false;
            }
        }
        true
    }

    /// Approximate false-positive probability given current fill level.
    pub fn false_positive_rate(&self) -> f64 {
        let m = self.bit_vec.len() as f64;
        let k = self.num_hashes as f64;
        let n = self.count as f64;
        (1.0 - (-k * n / m).exp()).powf(k)
    }

    /// Number of elements inserted (approximate — not decremented on logical "delete").
    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
