use primary_replica::ReplicationCluster;
use std::time::Duration;

fn main() {
    println!("=== Primary-Replica Replication Demo ===\n");
    println!("Cluster: 1 primary, 2 replicas\n");

    let cluster = ReplicationCluster::new(2);

    println!("Writing 3 keys to primary...");
    cluster.write("user:1", "Alice");
    cluster.write("user:2", "Bob");
    cluster.write("user:3", "Carol");

    println!("Reading from primary (immediate):");
    for k in &["user:1", "user:2", "user:3"] {
        println!("  {} = {:?}", k, cluster.read_from_primary(k));
    }

    println!("\nWaiting for replication...");
    std::thread::sleep(Duration::from_millis(100));

    for r in 0..cluster.replica_count() {
        println!("\nReading from replica-{}:", r);
        for k in &["user:1", "user:2", "user:3"] {
            println!("  {} = {:?}", k, cluster.read_from_replica(r, k));
        }
    }
}
