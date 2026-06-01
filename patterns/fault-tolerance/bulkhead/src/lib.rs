use std::sync::{Arc, Mutex};

/// A counting semaphore that limits concurrent access to a resource.
pub struct Bulkhead {
    available: Arc<Mutex<usize>>,
    capacity: usize,
}

impl Bulkhead {
    pub fn new(capacity: usize) -> Self {
        Self {
            available: Arc::new(Mutex::new(capacity)),
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn available(&self) -> usize {
        *self.available.lock().unwrap()
    }

    /// Acquire a permit. Returns Err immediately if no permits are free.
    pub fn acquire(&self) -> Result<Permit, BulkheadFull> {
        let mut n = self.available.lock().unwrap();
        if *n == 0 {
            return Err(BulkheadFull);
        }
        *n -= 1;
        Ok(Permit {
            available: self.available.clone(),
        })
    }

    /// Alias for acquire — non-blocking.
    pub fn try_acquire(&self) -> Result<Permit, BulkheadFull> {
        self.acquire()
    }
}

/// RAII permit — releases the slot when dropped.
pub struct Permit {
    available: Arc<Mutex<usize>>,
}

impl Drop for Permit {
    fn drop(&mut self) {
        *self.available.lock().unwrap() += 1;
    }
}

#[derive(Debug)]
pub struct BulkheadFull;

impl std::fmt::Display for BulkheadFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bulkhead full — request rejected")
    }
}
