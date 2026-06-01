use heartbeat::HeartbeatMonitor;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Heartbeat Monitor Demo ===\n");

    let monitor = Arc::new(Mutex::new(HeartbeatMonitor::new(Duration::from_millis(
        300,
    ))));
    for id in 1..=3 {
        monitor.lock().unwrap().register(id);
    }

    println!("Nodes 1-3 sending heartbeats every 100ms...");
    for id in 1..=3u64 {
        let m = monitor.clone();
        thread::spawn(move || {
            for _ in 0..5 {
                m.lock().unwrap().beat(id);
                thread::sleep(Duration::from_millis(100));
            }
            // Node 2 stops sending heartbeats after 5 beats
            if id == 2 {
                println!("  Node-2 stopped sending heartbeats");
            }
        });
    }

    for t in &[200u64, 400, 600] {
        thread::sleep(Duration::from_millis(200));
        let statuses = monitor.lock().unwrap().all_statuses();
        let mut ids: Vec<_> = statuses.keys().cloned().collect();
        ids.sort();
        print!("  t={}ms: ", t);
        for id in &ids {
            print!("node-{}: {:?}  ", id, statuses[id]);
        }
        println!();
    }
}
