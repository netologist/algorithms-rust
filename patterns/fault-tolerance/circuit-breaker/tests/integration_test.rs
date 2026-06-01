use circuit_breaker::{CircuitBreaker, CircuitState, Config};
use std::time::Duration;

#[test]
fn circuit_opens_after_threshold() {
    let mut cb = CircuitBreaker::new(Config {
        failure_threshold: 3,
        success_threshold: 2,
        half_open_timeout: Duration::from_millis(50),
    });
    for _ in 0..3 {
        cb.record_failure();
    }
    assert_eq!(cb.state(), CircuitState::Open);
}

#[test]
fn open_circuit_transitions_to_half_open_after_timeout() {
    let mut cb = CircuitBreaker::new(Config {
        failure_threshold: 1,
        success_threshold: 1,
        half_open_timeout: Duration::from_millis(50),
    });
    cb.record_failure();
    assert_eq!(cb.state(), CircuitState::Open);
    std::thread::sleep(Duration::from_millis(60));
    assert_eq!(cb.state(), CircuitState::HalfOpen);
}

#[test]
fn half_open_closes_after_success_threshold() {
    let mut cb = CircuitBreaker::new(Config {
        failure_threshold: 1,
        success_threshold: 2,
        half_open_timeout: Duration::from_millis(50),
    });
    cb.record_failure();
    std::thread::sleep(Duration::from_millis(60));
    assert_eq!(cb.state(), CircuitState::HalfOpen);
    cb.record_success();
    cb.record_success();
    assert_eq!(cb.state(), CircuitState::Closed);
}

#[test]
fn call_is_rejected_when_open() {
    let mut cb = CircuitBreaker::new(Config {
        failure_threshold: 1,
        success_threshold: 1,
        half_open_timeout: Duration::from_millis(1000),
    });
    cb.record_failure();
    let result: Result<i32, _> = cb.call(|| Ok::<i32, &str>(42));
    assert!(result.is_err());
}
