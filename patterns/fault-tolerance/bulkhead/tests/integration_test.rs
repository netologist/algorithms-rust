use bulkhead::Bulkhead;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

#[test]
fn admits_up_to_max_concurrent() {
    let bh = Arc::new(Bulkhead::new(3));
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];

    for _ in 0..3 {
        let bh = bh.clone();
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            let _permit = bh.acquire().unwrap();
            b.wait();
            thread::sleep(Duration::from_millis(50));
        }));
    }

    barrier.wait();
    assert_eq!(bh.available(), 0);
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(bh.available(), 3);
}

#[test]
fn rejects_when_full() {
    let bh = Arc::new(Bulkhead::new(2));
    let _p1 = bh.acquire().unwrap();
    let _p2 = bh.acquire().unwrap();
    assert!(bh.try_acquire().is_err());
}

#[test]
fn permit_released_on_drop() {
    let bh = Bulkhead::new(1);
    {
        let _p = bh.acquire().unwrap();
        assert_eq!(bh.available(), 0);
    }
    assert_eq!(bh.available(), 1);
}
