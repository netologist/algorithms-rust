use read_repair::ReadRepairCluster;
use std::time::Duration;

fn main() {
    println!("=== Read Repair Demo ===\n");
    println!("Cluster: 3 nodes\n");

    let cluster = ReadRepairCluster::new(3);

    println!("Writing 'config=v2' to all 3 nodes...");
    cluster.write("config", "v2");

    println!("Manually degrading node-2 to stale version...");
    cluster.write_stale(2, "config", "v1_stale", 0);

    println!("\nBefore read-repair:");
    for i in 0..3 {
        let v = cluster.read_from_node(i, "config");
        println!(
            "  node-{}: {:?}",
            i,
            v.map(|v| format!("'{}' v{}", v.value, v.version))
        );
    }

    println!("\nReading (triggers background read-repair for stale nodes)...");
    let result = cluster.read("config");
    println!("  Read result: {:?}", result);

    std::thread::sleep(Duration::from_millis(100));
    println!("\nAfter read-repair:");
    for i in 0..3 {
        let v = cluster.read_from_node(i, "config");
        println!(
            "  node-{}: {:?}",
            i,
            v.map(|v| format!("'{}' v{}", v.value, v.version))
        );
    }
}
