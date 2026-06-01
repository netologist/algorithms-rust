use gossip_protocol::GossipCluster;
use std::time::Duration;

fn main() {
    println!("=== Gossip Protocol Demo ===\n");
    println!("10-node cluster, fanout=3, 20 rounds × 25ms\n");

    let cluster = GossipCluster::new(10);
    cluster.start();

    println!("Infecting node-0 with rumour: news=Rust...");
    cluster.infect(0, "news", "Rust");

    for ms in &[100, 200, 300, 500] {
        std::thread::sleep(Duration::from_millis(100));
        let cov = cluster.coverage("news");
        println!("  t={}ms: coverage = {:.0}%", ms, cov * 100.0);
        if cov >= 1.0 {
            break;
        }
    }
}
