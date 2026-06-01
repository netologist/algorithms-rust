use range_partitioning::{RangePartitioner, Shard};

fn main() {
    println!("=== Range Partitioning Demo ===\n");
    let mut p = RangePartitioner::new(vec![
        Shard::new("shard-a", "", "f"),  // a-e
        Shard::new("shard-b", "f", "p"), // f-o
        Shard::new("shard-c", "p", ""),  // p-z
    ]);

    let keys = [
        "apple",
        "banana",
        "cherry",
        "fig",
        "grape",
        "kiwi",
        "pear",
        "plum",
        "strawberry",
        "zebra",
    ];
    println!("Key → Shard routing:");
    for k in &keys {
        println!("  {:12} → {}", k, p.shard_for(k));
    }

    println!("\nWriting all keys...");
    for (i, k) in keys.iter().enumerate() {
        p.write(k, &i.to_string());
    }
    println!("  grape = {:?}", p.read("grape"));
    println!("  pear  = {:?}", p.read("pear"));
}
