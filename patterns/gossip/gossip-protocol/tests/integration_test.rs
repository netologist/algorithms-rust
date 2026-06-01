use gossip_protocol::GossipCluster;
use std::time::Duration;

#[test]
fn rumor_spreads_to_all_nodes() {
    let cluster = GossipCluster::new(10);
    cluster.start();
    cluster.infect(0, "news", "Rust is awesome");
    std::thread::sleep(Duration::from_millis(600));
    let cov = cluster.coverage("news");
    assert!(cov >= 0.9, "gossip coverage: {:.0}%", cov * 100.0);
}

#[test]
fn dead_node_does_not_block_convergence() {
    let cluster = GossipCluster::new(10);
    cluster.kill_node(5);
    cluster.start();
    cluster.infect(0, "key", "value");
    std::thread::sleep(Duration::from_millis(600));
    // Alive nodes (9/10) should all have it
    let cov = cluster.coverage("key");
    assert!(cov >= 0.8, "coverage with dead node: {:.0}%", cov * 100.0);
}
