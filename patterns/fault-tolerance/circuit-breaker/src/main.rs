use circuit_breaker::{CircuitBreaker, Config};
use std::time::Duration;

fn main() {
    println!("=== Circuit Breaker Demo ===\n");

    let mut cb = CircuitBreaker::new(Config {
        failure_threshold: 3,
        success_threshold: 2,
        half_open_timeout: Duration::from_millis(300),
    });

    println!("[Closed] Simulating 3 failures...");
    for i in 1..=3 {
        cb.record_failure();
        println!("  Failure {}: state = {:?}", i, cb.state());
    }

    println!("\n[Open] Attempting call...");
    let result: Result<i32, circuit_breaker::CircuitError<&str>> = cb.call(|| Ok(42));
    println!("  Result: {:?} (rejected without calling service)", result);

    println!("\n[Waiting 350ms for half-open timeout...]");
    std::thread::sleep(Duration::from_millis(350));
    println!("  State: {:?}", cb.state());

    println!("\n[HalfOpen] Recording successes...");
    cb.record_success();
    println!("  1 success: state = {:?}", cb.state());
    cb.record_success();
    println!("  2 successes: state = {:?}", cb.state());

    println!("\n[Closed] Circuit reset. Calling service normally...");
    let result: Result<i32, circuit_breaker::CircuitError<&str>> = cb.call(|| Ok(42));
    println!("  Result: {:?}", result);
}
