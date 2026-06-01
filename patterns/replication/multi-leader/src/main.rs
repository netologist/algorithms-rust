use multi_leader::{ConflictResolution, MultiLeaderCluster};

fn main() {
    println!("=== Multi-Leader Replication Demo ===\n");
    let mut cluster = MultiLeaderCluster::new(2, ConflictResolution::LastWriteWins);

    println!("Leader-0 writes key='user' value='Alice' at ts=100");
    cluster.write(0, "user", "Alice", 100);

    println!("Leader-1 writes key='user' value='Bob'   at ts=200  ← CONFLICT");
    cluster.write(1, "user", "Bob", 200);

    println!("\nBefore sync:");
    println!("  Leader-0 sees: {:?}", cluster.read_from(0, "user"));
    println!("  Leader-1 sees: {:?}", cluster.read_from(1, "user"));

    println!("\nSyncing (LWW: higher timestamp wins)...");
    cluster.sync();

    println!("\nAfter sync:");
    println!("  Leader-0 sees: {:?}", cluster.read_from(0, "user"));
    println!("  Leader-1 sees: {:?}", cluster.read_from(1, "user"));
    println!("  → Both agree: 'Bob' (ts=200 won)");
}
