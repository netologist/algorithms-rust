use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ServiceInstance {
    pub id: String,
    pub address: String,
    pub registered_at: Instant,
    pub ttl: Duration,
}

impl ServiceInstance {
    pub fn is_expired(&self) -> bool {
        self.registered_at.elapsed() > self.ttl
    }
}

pub struct ServiceRegistry {
    services: HashMap<String, Vec<ServiceInstance>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, id: &str, address: &str, ttl: Duration) {
        self.services
            .entry(name.into())
            .or_default()
            .push(ServiceInstance {
                id: id.into(),
                address: address.into(),
                registered_at: Instant::now(),
                ttl,
            });
    }

    pub fn deregister(&mut self, name: &str, id: &str) {
        if let Some(instances) = self.services.get_mut(name) {
            instances.retain(|i| i.id != id);
        }
    }

    /// Return alive (non-expired) instances. Also evicts expired ones.
    pub fn lookup(&mut self, name: &str) -> Vec<ServiceInstance> {
        self.evict_expired();
        self.services.get(name).cloned().unwrap_or_default()
    }

    pub fn evict_expired(&mut self) {
        for instances in self.services.values_mut() {
            instances.retain(|i| !i.is_expired());
        }
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
