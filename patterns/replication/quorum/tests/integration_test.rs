use quorum::{QuorumCluster, QuorumConfig};

#[test]
fn strong_quorum_reads_latest_write() {
    // W=3, R=3, N=5 → W+R=6 > N=5
    let cluster = QuorumCluster::new(5, QuorumConfig { w: 3, r: 3 });
    cluster.write("k", "v1");
    let result = cluster.read("k");
    assert_eq!(result, Some("v1".into()));
}

#[test]
fn direct_node_write_visible_on_read() {
    let cluster = QuorumCluster::new(5, QuorumConfig { w: 3, r: 3 });
    cluster.write_to_node(0, "k", "v1", 1);
    cluster.write_to_node(1, "k", "v1", 1);
    cluster.write_to_node(2, "k", "v1", 1);
    // Node 3 and 4 are stale (no write)
    assert_eq!(cluster.read_from_node(0, "k"), Some("v1".into()));
    assert_eq!(cluster.read_from_node(3, "k"), None);
}

#[test]
fn multiple_writes_versioned() {
    let cluster = QuorumCluster::new(5, QuorumConfig { w: 5, r: 5 });
    cluster.write("counter", "1");
    cluster.write("counter", "2");
    cluster.write("counter", "3");
    let result = cluster.read("counter");
    assert_eq!(result, Some("3".into()));
}
