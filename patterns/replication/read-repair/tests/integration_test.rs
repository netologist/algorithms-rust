use read_repair::ReadRepairCluster;
use std::time::Duration;

#[test]
fn stale_node_repaired_after_read() {
    let cluster = ReadRepairCluster::new(3);
    cluster.write("k", "v1");
    // Manually stale node-2
    cluster.write_stale(2, "k", "old", 0);

    let result = cluster.read("k");
    assert_eq!(result, Some("v1".into()));

    std::thread::sleep(Duration::from_millis(50));
    let repaired = cluster.read_from_node(2, "k");
    assert_eq!(repaired.map(|v| v.value), Some("v1".into()));
}

#[test]
fn read_returns_latest_version() {
    let cluster = ReadRepairCluster::new(3);
    cluster.write("x", "first");
    cluster.write("x", "second");
    assert_eq!(cluster.read("x"), Some("second".into()));
}
