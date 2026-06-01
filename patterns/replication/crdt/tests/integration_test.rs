use crdt::{GCounter, LwwRegister};

#[test]
fn g_counter_merges_without_loss() {
    let mut a = GCounter::new(1);
    let mut b = GCounter::new(2);
    a.increment(3);
    b.increment(5);
    a.merge(&b);
    assert_eq!(a.value(), 8);
}

#[test]
fn g_counter_idempotent_merge() {
    let mut a = GCounter::new(1);
    a.increment(5);
    let b = a.clone();
    a.merge(&b);
    assert_eq!(a.value(), 5); // merging same state is idempotent
}

#[test]
fn g_counter_three_way_convergence() {
    let mut a = GCounter::new(1);
    let mut b = GCounter::new(2);
    let mut c = GCounter::new(3);
    a.increment(1);
    b.increment(2);
    c.increment(3);
    a.merge(&b);
    a.merge(&c);
    assert_eq!(a.value(), 6);
}

#[test]
fn lww_register_last_write_wins() {
    let mut r1 = LwwRegister::new(1);
    let mut r2 = LwwRegister::new(2);
    r1.set("hello", 100);
    r2.set("world", 200);
    r1.merge(&r2);
    assert_eq!(r1.get(), "world");
}

#[test]
fn lww_register_earlier_write_does_not_override() {
    let mut r1 = LwwRegister::new(1);
    let mut r2 = LwwRegister::new(2);
    r1.set("hello", 300);
    r2.set("world", 100);
    r1.merge(&r2);
    assert_eq!(r1.get(), "hello");
}
