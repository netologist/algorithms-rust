use std::time::Duration;
use timeout::{with_timeout, TimeoutError};

fn main() {
    println!("=== Timeout Pattern Demo ===\n");

    println!("Case 1: Fast operation (50ms) with 200ms deadline...");
    let r = with_timeout(Duration::from_millis(200), || {
        std::thread::sleep(Duration::from_millis(50));
        "fast response"
    });
    println!("  Result: {:?}\n", r);

    println!("Case 2: Slow operation (300ms) with 100ms deadline...");
    let r = with_timeout(Duration::from_millis(100), || {
        std::thread::sleep(Duration::from_millis(300));
        "slow response"
    });
    match r {
        Err(TimeoutError::Elapsed) => println!("  Result: TIMED OUT — deadline exceeded\n"),
        Ok(v) => println!("  Result: {:?}\n", v),
    }
}
