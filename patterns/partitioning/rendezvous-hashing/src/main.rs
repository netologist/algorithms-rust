use rendezvous_hashing::RendezvousHasher;

fn main() {
    println!("=== Rendezvous Hashing Demo ===\n");
    let nodes = (1..=5).map(|i| format!("node-{}", i)).collect();
    let mut h = RendezvousHasher::new(nodes);

    let keys: Vec<String> = (0..20).map(|i| format!("key-{:02}", i)).collect();
    println!("Key distribution (20 keys, {} nodes):", h.node_count());
    for k in &keys {
        println!("  {} → {}", k, h.get_node(k));
    }

    let before: Vec<String> = keys.iter().map(|k| h.get_node(k).into()).collect();
    println!("\nRemoving node-3...");
    h.remove_node("node-3");
    let after: Vec<String> = keys.iter().map(|k| h.get_node(k).into()).collect();
    let remapped = before
        .iter()
        .zip(after.iter())
        .filter(|(a, b)| a != b)
        .count();
    println!("Keys remapped: {}/20 (only node-3's keys moved)", remapped);
}
