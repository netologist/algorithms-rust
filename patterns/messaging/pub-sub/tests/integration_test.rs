use pub_sub::Broker;
use std::time::Duration;

#[test]
fn subscriber_receives_messages() {
    let mut broker = Broker::new();
    let rx = broker.subscribe("news");
    broker.publish("news", "headline 1");
    broker.publish("news", "headline 2");
    assert_eq!(
        rx.recv_timeout(Duration::from_millis(100)).unwrap(),
        "headline 1"
    );
    assert_eq!(
        rx.recv_timeout(Duration::from_millis(100)).unwrap(),
        "headline 2"
    );
}

#[test]
fn late_subscriber_misses_prior_messages() {
    let mut broker = Broker::new();
    broker.publish("news", "old news");
    let rx = broker.subscribe("news");
    assert!(rx.recv_timeout(Duration::from_millis(50)).is_err());
}

#[test]
fn multiple_subscribers_all_receive() {
    let mut broker = Broker::new();
    let rx1 = broker.subscribe("topic");
    let rx2 = broker.subscribe("topic");
    broker.publish("topic", "msg");
    assert_eq!(rx1.recv_timeout(Duration::from_millis(100)).unwrap(), "msg");
    assert_eq!(rx2.recv_timeout(Duration::from_millis(100)).unwrap(), "msg");
}

#[test]
fn different_topics_isolated() {
    let mut broker = Broker::new();
    let rx_a = broker.subscribe("topic-a");
    let rx_b = broker.subscribe("topic-b");
    broker.publish("topic-a", "for-a");
    assert_eq!(
        rx_a.recv_timeout(Duration::from_millis(50)).unwrap(),
        "for-a"
    );
    assert!(rx_b.recv_timeout(Duration::from_millis(50)).is_err());
}
