use bloom_filter::BloomFilter;

#[test]
fn inserted_items_are_always_found() {
    let mut bf = BloomFilter::new(1_000, 5);
    let items = ["alpha", "beta", "gamma", "delta", "epsilon"];
    for item in &items {
        bf.insert(item);
    }
    for item in &items {
        assert!(bf.may_contain(item), "should contain '{item}'");
    }
}

#[test]
fn uninserted_item_is_not_found_in_large_filter() {
    // With 10 000 bits for 3 items the FPR is negligible.
    let mut bf = BloomFilter::new(10_000, 7);
    bf.insert(&"a");
    bf.insert(&"b");
    bf.insert(&"c");
    assert!(!bf.may_contain(&"definitely-not-here-xyz-123"));
}

#[test]
fn len_tracks_inserts() {
    let mut bf = BloomFilter::new(1_000, 3);
    assert_eq!(bf.len(), 0);
    assert!(bf.is_empty());
    bf.insert(&"x");
    bf.insert(&"y");
    assert_eq!(bf.len(), 2);
    assert!(!bf.is_empty());
}

#[test]
fn false_positive_rate_is_low_for_well_sized_filter() {
    // m = 9585, k = 7, n ≤ 1000 → theoretical FPR ≈ 1%
    let mut bf = BloomFilter::new(9_585, 7);
    for i in 0..1_000u32 {
        bf.insert(&i);
    }
    assert!(
        bf.false_positive_rate() < 0.02,
        "FPR too high: {:.4}",
        bf.false_positive_rate()
    );
}
