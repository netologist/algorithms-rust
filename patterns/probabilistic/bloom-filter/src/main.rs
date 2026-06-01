use bloom_filter::BloomFilter;

fn main() {
    // Optimal for ~1 000 items with ~1% false-positive rate:
    //   m = -n * ln(p) / (ln 2)^2 ≈ 9585 bits
    //   k =  m / n * ln(2)        ≈ 7 hashes
    let mut bf = BloomFilter::new(9_585, 7);

    let fruits = ["apple", "banana", "cherry", "date", "elderberry"];
    for fruit in &fruits {
        bf.insert(fruit);
    }

    println!("=== Bloom Filter Demo ===");
    println!("Inserted: {:?}", fruits);
    println!();

    for word in &["apple", "banana", "durian", "fig", "cherry"] {
        let result = if bf.may_contain(word) {
            "possibly YES"
        } else {
            "definitely NO"
        };
        println!("  '{word}' → {result}");
    }

    println!();
    println!("Elements inserted : {}", bf.len());
    println!(
        "Est. false-pos rate: {:.4}%",
        bf.false_positive_rate() * 100.0
    );
}
