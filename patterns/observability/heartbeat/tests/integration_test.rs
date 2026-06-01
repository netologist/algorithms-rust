use heartbeat::{HeartbeatMonitor, NodeStatus};
use std::time::Duration;

#[test]
fn healthy_node_stays_up() {
    let mut m = HeartbeatMonitor::new(Duration::from_millis(100));
    m.register(1);
    m.beat(1);
    std::thread::sleep(Duration::from_millis(30));
    assert_eq!(m.status(1), NodeStatus::Up);
}

#[test]
fn missed_beat_marks_node_down() {
    let mut m = HeartbeatMonitor::new(Duration::from_millis(50));
    m.register(1);
    m.beat(1);
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(m.status(1), NodeStatus::Down);
}

#[test]
fn unregistered_node_is_down() {
    let m = HeartbeatMonitor::new(Duration::from_millis(100));
    assert_eq!(m.status(99), NodeStatus::Down);
}

#[test]
fn beat_resets_timeout() {
    let mut m = HeartbeatMonitor::new(Duration::from_millis(80));
    m.register(1);
    m.beat(1);
    std::thread::sleep(Duration::from_millis(60));
    m.beat(1); // refresh
    std::thread::sleep(Duration::from_millis(60));
    assert_eq!(m.status(1), NodeStatus::Up);
}
