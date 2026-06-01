use crdt::{GCounter, LwwRegister};

fn main() {
    println!("=== CRDT Demo ===\n");

    println!("--- G-Counter (Grow-Only Counter) ---");
    let mut node1 = GCounter::new(1);
    let mut node2 = GCounter::new(2);
    let mut node3 = GCounter::new(3);

    node1.increment(5);
    node2.increment(3);
    node3.increment(7);
    println!("Node-1 increments by 5, Node-2 by 3, Node-3 by 7");
    println!(
        "Before merge: node1={}, node2={}, node3={}",
        node1.value(),
        node2.value(),
        node3.value()
    );

    node1.merge(&node2);
    node1.merge(&node3);
    println!("After node1 merges all: node1={} (5+3+7=15)", node1.value());

    println!("\n--- LWW-Register (Last-Write-Wins) ---");
    let mut r1 = LwwRegister::new(1);
    let mut r2 = LwwRegister::new(2);

    r1.set("Paris", 100);
    r2.set("Berlin", 200);
    println!("r1 sets 'Paris'  at ts=100");
    println!("r2 sets 'Berlin' at ts=200");

    r1.merge(&r2);
    println!("After r1.merge(r2): {:?} (ts=200 wins)", r1.get());
}
