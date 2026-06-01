use three_phase_commit::Coordinator;

fn main() {
    println!("=== Three-Phase Commit Demo ===\n");

    println!("--- Scenario 1: Happy path (all 3 phases complete) ---");
    let coord = Coordinator::new(3, false);
    let d = coord.run();
    println!("  → Decision: {:?}\n", d);

    println!("--- Scenario 2: Coordinator crashes after Phase 1 ---");
    let coord = Coordinator::new(3, true);
    let d = coord.run();
    println!("  → Decision: {:?}", d);
}
