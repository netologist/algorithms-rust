use paxos::PaxosCluster;

fn main() {
    println!("=== Paxos (Single-Decree) Demo ===\n");

    let mut cluster = PaxosCluster::new(5);

    println!("Round 1: Proposer suggests value=42\n");
    match cluster.propose(42) {
        Ok(v) => println!("✓ Consensus reached: {}\n", v),
        Err(e) => println!("✗ Failed: {:?}\n", e),
    }

    println!("Round 2: Proposer suggests value=99");
    println!("(Acceptors already accepted 42 — Paxos must return 42)\n");
    match cluster.propose(99) {
        Ok(v) => println!("✓ Consensus reached: {} (prior value preserved)", v),
        Err(e) => println!("✗ Failed: {:?}", e),
    }
}
