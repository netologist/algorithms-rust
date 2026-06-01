use leader_election::BullyCluster;

fn main() {
    println!("=== Leader Election (Bully Algorithm) Demo ===\n");

    println!("Starting 5-node cluster and running initial election...");
    let mut cluster = BullyCluster::new(5);
    println!("  Leader: node-{}\n", cluster.current_leader().unwrap());

    println!("Killing the current leader (node-5)...");
    cluster.kill_node(5);
    println!("  New leader: node-{}\n", cluster.current_leader().unwrap());

    println!("Killing node-4 as well...");
    cluster.kill_node(4);
    println!("  New leader: node-{}\n", cluster.current_leader().unwrap());
}
