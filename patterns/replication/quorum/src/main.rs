use quorum::{QuorumCluster, QuorumConfig};

fn main() {
    println!("=== Quorum Read/Write Demo ===\n");
    println!("Cluster: N=5 nodes, W=3, R=3 (W+R=6 > N=5 → strong consistency)\n");

    let cluster = QuorumCluster::new(5, QuorumConfig { w: 3, r: 3 });

    println!("Writing 'user=Alice' to 3 of 5 nodes...");
    cluster.write("user", "Alice");
    println!("  Read result: {:?}\n", cluster.read("user"));

    println!("Manually writing stale data to nodes 3 & 4...");
    cluster.write_to_node(3, "user", "STALE", 0);
    cluster.write_to_node(4, "user", "STALE", 0);

    println!("  Node-0: {:?}", cluster.read_from_node(0, "user"));
    println!("  Node-3: {:?} (stale)", cluster.read_from_node(3, "user"));

    println!("\nQuorum read (R=3 picks highest version):");
    println!("  Result: {:?} (latest version wins)", cluster.read("user"));
}
