use leader_election::BullyCluster;

#[test]
fn highest_id_becomes_leader() {
    let cluster = BullyCluster::new(5);
    assert_eq!(cluster.current_leader(), Some(5));
}

#[test]
fn new_election_on_leader_death() {
    let mut cluster = BullyCluster::new(5);
    assert_eq!(cluster.current_leader(), Some(5));
    cluster.kill_node(5);
    assert_eq!(cluster.current_leader(), Some(4));
}

#[test]
fn cascading_failures_elect_survivor() {
    let mut cluster = BullyCluster::new(5);
    cluster.kill_node(5);
    cluster.kill_node(4);
    cluster.kill_node(3);
    assert_eq!(cluster.current_leader(), Some(2));
}
