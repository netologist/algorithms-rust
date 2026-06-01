use bulkhead::Bulkhead;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Bulkhead Demo ===\n");
    println!("Bulkhead capacity: 3 concurrent slots\n");

    let bh = Arc::new(Bulkhead::new(3));
    let mut handles = vec![];

    for i in 1..=5 {
        let bh = bh.clone();
        handles.push(thread::spawn(move || match bh.try_acquire() {
            Ok(_permit) => {
                println!("  Request {}: ADMITTED  (slot acquired)", i);
                thread::sleep(Duration::from_millis(200));
                println!("  Request {}: DONE      (slot released)", i);
            }
            Err(_) => {
                println!("  Request {}: REJECTED  (bulkhead full)", i);
            }
        }));
        thread::sleep(Duration::from_millis(10));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!(
        "\nFinal available slots: {}/{}",
        bh.available(),
        bh.capacity()
    );
}
