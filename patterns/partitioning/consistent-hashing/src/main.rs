use consistent_hashing::HashRing;

fn main() {
    println!("=== Consistent Hashing Demo ===\n");
    println!("Ring: 5 nodes, 150 virtual nodes each\n");

    let mut ring = HashRing::new(150);
    for i in 1..=5 {
        ring.add_node(&format!("node-{}", i));
    }

    let keys: Vec<String> = (0..20).map(|i| format!("key-{:02}", i)).collect();
    println!(
        "Key distribution (20 keys across {} nodes):",
        ring.node_count()
    );
    for k in &keys {
        println!("  {} → {}", k, ring.get_node(k));
    }

    let before: Vec<String> = keys.iter().map(|k| ring.get_node(k).into()).collect();
    println!("\nRemoving node-3...");
    ring.remove_node("node-3");

    let after: Vec<String> = keys.iter().map(|k| ring.get_node(k).into()).collect();
    let remapped = before
        .iter()
        .zip(after.iter())
        .filter(|(a, b)| a != b)
        .count();
    println!("Keys remapped: {}/20 (expected ~4, i.e. 1/5)", remapped);
}
