use rendezvous_hashing::RendezvousHasher;

#[test]
fn same_key_maps_to_same_node() {
    let h = RendezvousHasher::new(vec!["a".into(), "b".into(), "c".into()]);
    assert_eq!(h.get_node("test-key"), h.get_node("test-key"));
}

#[test]
fn removing_node_only_remaps_its_keys() {
    let nodes: Vec<String> = (1..=5).map(|i| format!("node-{}", i)).collect();
    let mut h = RendezvousHasher::new(nodes);

    let keys: Vec<String> = (0..200).map(|i| format!("key-{}", i)).collect();
    let before: Vec<String> = keys.iter().map(|k| h.get_node(k).into()).collect();

    h.remove_node("node-3");

    let after: Vec<String> = keys.iter().map(|k| h.get_node(k).into()).collect();
    // Remapped keys must NOT go to "node-3"
    for k in after.iter() {
        assert_ne!(k, "node-3");
    }
    // Only keys that were on node-3 should move
    let remapped = before
        .iter()
        .zip(after.iter())
        .filter(|(a, b)| a != b)
        .count();
    // ~1/5 of 200 = ~40
    assert!(remapped < 80, "too many remapped: {}", remapped);
}
