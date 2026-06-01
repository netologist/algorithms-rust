use wal::{KvStore, WriteAheadLog};

#[test]
fn replay_recovers_state_after_crash() {
    let log = WriteAheadLog::new();
    log.append("SET x 1");
    log.append("SET y 2");
    log.append("SET x 3");

    let mut store = KvStore::new();
    store.recover_from(&log);

    assert_eq!(store.get("x"), Some("3"));
    assert_eq!(store.get("y"), Some("2"));
}

#[test]
fn uncommitted_entry_ignored_on_recovery() {
    let log = WriteAheadLog::new();
    log.append("SET a 1");
    log.append_uncommitted("SET a 999");

    let mut store = KvStore::new();
    store.recover_from(&log);

    assert_eq!(store.get("a"), Some("1"));
}

#[test]
fn empty_log_gives_empty_store() {
    let log = WriteAheadLog::new();
    let mut store = KvStore::new();
    store.recover_from(&log);
    assert_eq!(store.get("anything"), None);
}
