use consistent_hashing::HashRing;

#[test]
fn keys_routed_to_a_node() {
    let mut ring = HashRing::new(150);
    ring.add_node("node-1");
    ring.add_node("node-2");
    ring.add_node("node-3");
    let node = ring.get_node("my-key");
    assert!(["node-1", "node-2", "node-3"].contains(&node));
}

#[test]
fn same_key_always_routes_to_same_node() {
    let mut ring = HashRing::new(150);
    ring.add_node("node-1");
    ring.add_node("node-2");
    ring.add_node("node-3");
    let first = ring.get_node("stable-key").to_string();
    let second = ring.get_node("stable-key").to_string();
    assert_eq!(first, second);
}

#[test]
fn minimal_keys_remapped_on_node_removal() {
    let mut ring = HashRing::new(150);
    for i in 1..=5 {
        ring.add_node(&format!("node-{}", i));
    }

    let keys: Vec<String> = (0..1000).map(|i| format!("key-{}", i)).collect();
    let before: Vec<String> = keys.iter().map(|k| ring.get_node(k).into()).collect();

    ring.remove_node("node-3");

    let after: Vec<String> = keys.iter().map(|k| ring.get_node(k).into()).collect();
    let remapped = before
        .iter()
        .zip(after.iter())
        .filter(|(a, b)| a != b)
        .count();

    // ~20% should move (1 of 5 nodes removed)
    assert!(remapped < 350, "too many keys remapped: {}", remapped);
}
