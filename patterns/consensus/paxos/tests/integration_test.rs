use paxos::PaxosCluster;

#[test]
fn single_decree_consensus_reached() {
    let mut cluster = PaxosCluster::new(5);
    let result = cluster.propose(42);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn previously_accepted_value_takes_precedence() {
    let mut cluster = PaxosCluster::new(5);
    // First round: propose 100, it gets accepted
    cluster.propose(100).unwrap();
    // Second round: propose 200, but acceptors already accepted 100
    // Paxos must return 100 (the already-accepted value)
    let result = cluster.propose(200).unwrap();
    assert_eq!(result, 100, "already-accepted value must win");
}
