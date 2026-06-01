use load_balancer::{Algorithm, LoadBalancer};
use std::collections::HashMap;

fn main() {
    println!("=== Load Balancer Demo ===\n");
    let backends = vec!["backend-1".into(), "backend-2".into(), "backend-3".into()];
    let n = 9;

    for algo in [
        Algorithm::RoundRobin,
        Algorithm::LeastConnections,
        Algorithm::ConsistentHash,
    ] {
        let mut lb = LoadBalancer::new(backends.clone(), algo.clone());
        if matches!(algo, Algorithm::LeastConnections) {
            lb.connect("backend-1");
            lb.connect("backend-1");
            lb.connect("backend-2");
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        let keys = [
            "user:1", "user:2", "user:3", "user:4", "user:5", "user:6", "user:7", "user:8",
            "user:9",
        ];
        for k in keys.iter().take(n) {
            let b = lb.select(Some(k)).to_string();
            *counts.entry(b).or_insert(0) += 1;
        }
        println!("{:?}: {:?}", algo, counts);
    }
}
