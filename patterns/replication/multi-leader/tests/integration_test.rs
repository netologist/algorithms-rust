use multi_leader::{ConflictResolution, MultiLeaderCluster};

#[test]
fn concurrent_writes_lww_higher_ts_wins() {
    let mut cluster = MultiLeaderCluster::new(2, ConflictResolution::LastWriteWins);
    cluster.write(0, "key", "from-leader-0", 100);
    cluster.write(1, "key", "from-leader-1", 200);
    cluster.sync();
    assert_eq!(cluster.read("key"), Some("from-leader-1"));
}

#[test]
fn no_conflict_both_values_survive() {
    let mut cluster = MultiLeaderCluster::new(2, ConflictResolution::LastWriteWins);
    cluster.write(0, "a", "from-0", 100);
    cluster.write(1, "b", "from-1", 100);
    cluster.sync();
    assert_eq!(cluster.read("a"), Some("from-0"));
    assert_eq!(cluster.read("b"), Some("from-1"));
}

#[test]
fn all_leaders_converge_after_sync() {
    let mut cluster = MultiLeaderCluster::new(3, ConflictResolution::LastWriteWins);
    cluster.write(0, "k", "v0", 50);
    cluster.write(1, "k", "v1", 150);
    cluster.write(2, "k", "v2", 100);
    cluster.sync();
    // All should agree on v1 (highest ts)
    assert_eq!(cluster.read_from(0, "k"), Some("v1"));
    assert_eq!(cluster.read_from(1, "k"), Some("v1"));
    assert_eq!(cluster.read_from(2, "k"), Some("v1"));
}
