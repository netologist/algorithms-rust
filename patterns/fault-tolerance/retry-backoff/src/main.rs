use retry_backoff::{retry, RetryConfig};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    println!("=== Retry with Exponential Backoff Demo ===\n");

    let attempts = Arc::new(AtomicU32::new(0));
    let a = attempts.clone();

    let cfg = RetryConfig {
        max_retries: 5,
        base_delay: Duration::from_millis(50),
        max_delay: Duration::from_secs(2),
        jitter: true,
    };

    println!("Calling an unreliable service (fails 3 times before succeeding)...");
    let result: Result<&str, &str> = retry(cfg, move || {
        let n = a.fetch_add(1, Ordering::SeqCst) + 1;
        if n <= 3 {
            println!("  Attempt {}: FAILED — retrying with backoff...", n);
            Err("service unavailable")
        } else {
            println!("  Attempt {}: SUCCESS", n);
            Ok("response data")
        }
    });

    match result {
        Ok(v) => println!("\nFinal result: {:?}", v),
        Err(e) => println!("\nAll retries exhausted. Last error: {:?}", e),
    }

    println!("\n--- Exhausted retries demo ---");
    let result: Result<i32, &str> = retry(
        RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(20),
            jitter: false,
            ..Default::default()
        },
        || {
            println!("  Attempt: FAILED");
            Err("always down")
        },
    );
    println!("Result: {:?}", result);
}
