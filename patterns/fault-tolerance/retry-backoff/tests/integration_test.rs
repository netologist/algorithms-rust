use retry_backoff::{retry, RetryConfig};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn succeeds_on_first_try() {
    let result = retry(RetryConfig::default(), || Ok::<i32, &str>(42));
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn retries_until_success() {
    let attempts = Arc::new(AtomicU32::new(0));
    let a = attempts.clone();
    let result = retry(
        RetryConfig {
            max_retries: 5,
            base_delay: Duration::from_millis(1),
            jitter: false,
            ..Default::default()
        },
        move || {
            let n = a.fetch_add(1, Ordering::SeqCst);
            if n < 3 {
                Err("not yet")
            } else {
                Ok(n)
            }
        },
    );
    assert!(result.is_ok());
    assert_eq!(attempts.load(Ordering::SeqCst), 4);
}

#[test]
fn fails_after_max_retries() {
    let result = retry(
        RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(1),
            jitter: false,
            ..Default::default()
        },
        || Err::<i32, &str>("always fails"),
    );
    assert!(result.is_err());
}

#[test]
fn delay_grows_exponentially() {
    let cfg = RetryConfig {
        max_retries: 4,
        base_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        jitter: false,
    };
    assert_eq!(cfg.delay_for(0), Duration::from_millis(10));
    assert_eq!(cfg.delay_for(1), Duration::from_millis(20));
    assert_eq!(cfg.delay_for(2), Duration::from_millis(40));
    assert_eq!(cfg.delay_for(3), Duration::from_millis(80));
}
