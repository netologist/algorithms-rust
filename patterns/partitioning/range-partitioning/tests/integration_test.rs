use range_partitioning::{RangePartitioner, Shard};

#[test]
fn keys_routed_to_correct_shard() {
    let mut p = RangePartitioner::new(vec![
        Shard::new("shard-a", "", "m"),  // [-, m)
        Shard::new("shard-b", "m", "t"), // [m, t)
        Shard::new("shard-c", "t", ""),  // [t, +)
    ]);
    assert_eq!(p.write("apple", "1"), "shard-a");
    assert_eq!(p.write("mango", "2"), "shard-b");
    assert_eq!(p.write("tomato", "3"), "shard-c");
}

#[test]
fn read_returns_written_value() {
    let mut p = RangePartitioner::new(vec![
        Shard::new("shard-a", "", "m"),
        Shard::new("shard-b", "m", ""),
    ]);
    p.write("banana", "yellow");
    assert_eq!(p.read("banana"), Some("yellow"));
}

#[test]
fn shard_for_returns_correct_name() {
    let p = RangePartitioner::new(vec![
        Shard::new("low", "", "f"),
        Shard::new("mid", "f", "p"),
        Shard::new("high", "p", ""),
    ]);
    assert_eq!(p.shard_for("cat"), "low");
    assert_eq!(p.shard_for("kiwi"), "mid");
    assert_eq!(p.shard_for("strawberry"), "high");
}
