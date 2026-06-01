use outbox::{relay, Database, InMemoryBroker};

fn main() {
    println!("=== Transactional Outbox Demo ===\n");

    let mut db = Database::new();
    let mut broker = InMemoryBroker::new();

    println!("Writing 3 records with outbox entries (atomic)...");
    db.write_with_outbox(
        "order:1",
        "created",
        "orders",
        r#"{"id":1,"status":"created"}"#,
    );
    db.write_with_outbox(
        "order:2",
        "created",
        "orders",
        r#"{"id":2,"status":"created"}"#,
    );
    db.write_with_outbox(
        "order:3",
        "created",
        "orders",
        r#"{"id":3,"status":"created"}"#,
    );

    println!("Pending outbox entries: {}", db.pending_outbox().len());

    println!("\nRunning relay (first poll)...");
    relay(&mut db, &mut broker);
    println!("Pending after relay: {}", db.pending_outbox().len());

    println!("\nRunning relay again (second poll — nothing to publish)...");
    relay(&mut db, &mut broker);

    println!(
        "\nBroker received {} messages total",
        broker.published.len()
    );
}
