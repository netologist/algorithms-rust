use load_balancer::{Algorithm, LoadBalancer};
use std::collections::HashMap;

#[test]
fn round_robin_cycles_evenly() {
    let mut lb = LoadBalancer::new(
        vec!["b1".into(), "b2".into(), "b3".into()],
        Algorithm::RoundRobin,
    );
    let selections: Vec<String> = (0..6).map(|_| lb.select(None).into()).collect();
    assert_eq!(selections, vec!["b1", "b2", "b3", "b1", "b2", "b3"]);
}

#[test]
fn least_connections_picks_least_loaded() {
    let mut lb = LoadBalancer::new(
        vec!["b1".into(), "b2".into(), "b3".into()],
        Algorithm::LeastConnections,
    );
    lb.connect("b1");
    lb.connect("b1");
    lb.connect("b2");
    // b3 has 0 connections → should be selected
    assert_eq!(lb.select(None), "b3");
}

#[test]
fn consistent_hash_same_key_same_backend() {
    let mut lb = LoadBalancer::new(
        vec!["b1".into(), "b2".into(), "b3".into()],
        Algorithm::ConsistentHash,
    );
    let first = lb.select(Some("user:42")).to_string();
    let second = lb.select(Some("user:42")).to_string();
    assert_eq!(first, second);
}

#[test]
fn connect_disconnect_tracks_connections() {
    let mut lb = LoadBalancer::new(vec!["b1".into()], Algorithm::RoundRobin);
    lb.connect("b1");
    lb.connect("b1");
    assert_eq!(lb.backends()[0].active_connections, 2);
    lb.disconnect("b1");
    assert_eq!(lb.backends()[0].active_connections, 1);
}
