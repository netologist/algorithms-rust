use chord_dht::ChordRing;

#[test]
fn key_lookup_finds_correct_node() {
    let mut ring = ChordRing::new(4); // 16-node ring
    for id in [0u64, 4, 8, 12] {
        ring.join(id);
    }
    ring.build_finger_tables();
    ring.put(2, "hello");
    assert_eq!(ring.get(2), Some("hello"));
}

#[test]
fn lookup_hop_count_bounded_by_m() {
    let mut ring = ChordRing::new(4);
    for id in [0u64, 2, 5, 9, 12] {
        ring.join(id);
    }
    ring.build_finger_tables();
    let (_, hops) = ring.lookup(7);
    assert!(hops <= 5, "hops should be ≤ M+1=5, got {}", hops);
}

#[test]
fn all_puts_retrievable() {
    let mut ring = ChordRing::new(5);
    for id in [0u64, 8, 16, 24] {
        ring.join(id);
    }
    ring.build_finger_tables();
    for k in 0u64..10 {
        ring.put(k, "v");
        assert_eq!(ring.get(k), Some("v"));
    }
}
