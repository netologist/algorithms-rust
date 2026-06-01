use raft::{RaftCluster, RaftConfig, RaftRole, node_role};
use std::time::Duration;

#[tokio::test]
async fn cluster_elects_exactly_one_leader() {
    let cluster = RaftCluster::new(5, RaftConfig::default()).await;
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert_eq!(cluster.leader_count(), 1, "expected exactly 1 leader");
}

#[tokio::test]
async fn leader_has_leader_role() {
    let cluster = RaftCluster::new(5, RaftConfig::default()).await;
    tokio::time::sleep(Duration::from_millis(600)).await;
    let leader_id = cluster.leader_id();
    assert!(leader_id.is_some());
    let idx = (leader_id.unwrap() - 1) as usize;
    assert_eq!(node_role(&cluster, idx), RaftRole::Leader);
}
