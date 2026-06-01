use snowflake::SnowflakeGenerator;

fn main() {
    // Datacenter 1, Machine 3
    let gen = SnowflakeGenerator::new(1, 3);

    println!("=== Snowflake ID Demo ===\n");
    println!(
        "{:<20} {:>15} {:>12} {:>12} {:>10}",
        "ID", "timestamp_ms", "datacenter", "machine", "sequence"
    );
    println!("{}", "-".repeat(75));

    for _ in 0..10 {
        let id = gen.next_id();
        let parsed = SnowflakeGenerator::parse(id);
        println!(
            "{:<20} {:>15} {:>12} {:>12} {:>10}",
            id, parsed.timestamp_ms, parsed.datacenter_id, parsed.machine_id, parsed.sequence,
        );
    }
}
