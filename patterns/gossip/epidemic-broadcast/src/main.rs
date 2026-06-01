use epidemic_broadcast::EpidemicCluster;
use std::time::Duration;

fn main() {
    println!("=== Epidemic Broadcast (SI Model) Demo ===\n");

    let cluster = EpidemicCluster::new(10);
    println!("10 nodes, 1 dead (node-5)");
    cluster.kill_node(5);

    println!("Broadcasting 'alert' from node-0...\n");
    cluster.broadcast("alert");

    for ms in &[100u64, 200, 300, 500] {
        std::thread::sleep(Duration::from_millis(100));
        let infected = cluster.infected_count();
        let alive = cluster.alive_count();
        println!(
            "  t={}ms: {}/{} alive nodes infected",
            ms,
            infected.min(alive),
            alive
        );
        if infected >= alive {
            break;
        }
    }
}
