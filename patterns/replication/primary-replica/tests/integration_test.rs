use primary_replica::ReplicationCluster;
use std::time::Duration;

#[test]
fn write_to_primary_replicated_to_all() {
    let cluster = ReplicationCluster::new(2);
    cluster.write("key1", "value1");
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(cluster.read_from_replica(0, "key1"), Some("value1".into()));
    assert_eq!(cluster.read_from_replica(1, "key1"), Some("value1".into()));
}

#[test]
fn primary_serves_reads_immediately() {
    let cluster = ReplicationCluster::new(2);
    cluster.write("x", "10");
    assert_eq!(cluster.read_from_primary("x"), Some("10".into()));
}

#[test]
fn replica_eventually_consistent() {
    let cluster = ReplicationCluster::new(2);
    cluster.write("y", "hello");
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(cluster.read_from_replica(0, "y"), Some("hello".into()));
}
