use wal::{KvStore, WriteAheadLog};

fn main() {
    println!("=== Write-Ahead Log Demo ===\n");

    let log = WriteAheadLog::new();
    println!("Writing 3 operations to WAL...");
    log.append("SET user Alice");
    log.append("SET score 100");
    log.append("SET user Bob");

    println!("Simulating crash (appending uncommitted entry)...");
    log.append_uncommitted("SET score 999");

    println!("\nWAL contents:");
    for e in log.all_entries() {
        println!("  [seq={} {:?}] {}", e.sequence, e.status, e.data);
    }

    println!("\nRecovering store from WAL (committed entries only)...");
    let mut store = KvStore::new();
    store.recover_from(&log);

    println!("Recovered state:");
    println!("  user  = {:?}", store.get("user"));
    println!(
        "  score = {:?} (uncommitted write discarded)",
        store.get("score")
    );
}
