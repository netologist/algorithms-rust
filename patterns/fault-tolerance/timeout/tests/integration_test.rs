use std::time::Duration;
use timeout::{with_timeout, TimeoutError};

#[test]
fn completes_within_deadline() {
    let result = with_timeout(Duration::from_millis(200), || {
        std::thread::sleep(Duration::from_millis(50));
        42i32
    });
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn times_out_when_slow() {
    let result = with_timeout(Duration::from_millis(50), || {
        std::thread::sleep(Duration::from_millis(300));
        42i32
    });
    assert!(matches!(result, Err(TimeoutError::Elapsed)));
}
