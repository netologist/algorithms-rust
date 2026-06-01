use outbox::{relay, Database, InMemoryBroker};

#[test]
fn write_creates_outbox_entry() {
    let mut db = Database::new();
    db.write_with_outbox("user:1", "Alice", "user-events", r#"{"event":"created"}"#);
    assert_eq!(db.pending_outbox().len(), 1);
    assert_eq!(db.records.get("user:1").unwrap(), "Alice");
}

#[test]
fn relay_publishes_and_marks_done() {
    let mut db = Database::new();
    let mut broker = InMemoryBroker::new();

    db.write_with_outbox("k", "v", "topic", "payload");
    assert_eq!(db.pending_outbox().len(), 1);

    relay(&mut db, &mut broker);

    assert_eq!(broker.published.len(), 1);
    assert_eq!(db.pending_outbox().len(), 0); // all published
}

#[test]
fn relay_does_not_republish() {
    let mut db = Database::new();
    let mut broker = InMemoryBroker::new();

    db.write_with_outbox("k", "v", "topic", "payload");
    relay(&mut db, &mut broker);
    relay(&mut db, &mut broker); // second poll

    assert_eq!(broker.published.len(), 1); // not duplicated
}
