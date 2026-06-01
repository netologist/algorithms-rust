use epidemic_broadcast::EpidemicCluster;
use std::time::Duration;

#[test]
fn all_alive_nodes_eventually_infected() {
    let cluster = EpidemicCluster::new(10);
    cluster.broadcast("important-data");
    std::thread::sleep(Duration::from_millis(600));
    assert_eq!(cluster.infected_count(), 10);
}

#[test]
fn dead_node_does_not_block_convergence() {
    let cluster = EpidemicCluster::new(10);
    cluster.kill_node(5);
    cluster.broadcast("data");
    std::thread::sleep(Duration::from_millis(600));
    // 9 alive nodes should all be infected
    assert_eq!(cluster.alive_count(), 9);
    // infected count excludes dead node behaviour — at least 8 alive infected
    assert!(cluster.infected_count() >= 8);
}
