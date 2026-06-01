use chord_dht::ChordRing;

fn main() {
    println!("=== Chord DHT Demo ===\n");
    println!("M=4 bit ring (size=16), 5 nodes\n");

    let mut ring = ChordRing::new(4);
    for id in [0u64, 3, 7, 11, 14] {
        ring.join(id);
        println!("  Joined node-{}", id);
    }
    ring.build_finger_tables();

    println!("\nFinger tables:");
    for (&id, node) in &ring.nodes {
        println!(
            "  node-{}: successor={}, fingers={:?}",
            id, node.successor, node.finger_table
        );
    }

    println!("\nStoring keys 1..10:");
    for k in 1u64..=10 {
        ring.put(k, "data");
        let (owner, hops) = ring.lookup(k);
        println!("  key={:2} → node-{} ({} hops)", k, owner, hops);
    }
}
