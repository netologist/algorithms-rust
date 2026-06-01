use raft::{RaftCluster, RaftConfig};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("=== Raft Consensus Demo ===\n");
    println!("Starting 5-node Raft cluster...");

    let cluster = RaftCluster::new(5, RaftConfig::default()).await;

    println!("Waiting for leader election...");
    tokio::time::sleep(Duration::from_millis(600)).await;

    match cluster.leader_id() {
        Some(id) => println!("✓ Leader elected: node-{}", id),
        None => println!("✗ No leader elected yet"),
    }
    println!("  Leaders: {}/5\n", cluster.leader_count());

    println!("Appending 3 log entries via leader...");
    for i in 1..=3 {
        cluster.append(format!("command-{}", i)).unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    tokio::time::sleep(Duration::from_millis(200)).await;
    let entries = cluster.committed_entries();
    println!("Committed entries: {}", entries.len());
    for e in &entries {
        println!("  [term={}, idx={}] {}", e.term, e.index, e.data);
    }

    println!("\nKilling current leader...");
    if let Some(leader_id) = cluster.leader_id() {
        cluster.kill_node(leader_id);
        println!("  Killed node-{}", leader_id);
    }

    println!("Waiting for re-election...");
    tokio::time::sleep(Duration::from_millis(800)).await;

    match cluster.leader_id() {
        Some(id) => println!("✓ New leader elected: node-{}", id),
        None => println!("  (re-election in progress...)"),
    }
}
