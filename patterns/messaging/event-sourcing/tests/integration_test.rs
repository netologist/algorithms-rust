use event_sourcing::{AccountEvent, AccountState, EventStore, Snapshot};

#[test]
fn state_rebuilt_from_events() {
    let mut store = EventStore::new();
    store.append(AccountEvent::Deposited { amount: 100 });
    store.append(AccountEvent::Deposited { amount: 50 });
    store.append(AccountEvent::Withdrawn { amount: 30 });

    let state = AccountState::from_events(store.events());
    assert_eq!(state.balance, 120);
}

#[test]
fn snapshot_plus_partial_replay_same_result() {
    let mut store = EventStore::new();
    store.append(AccountEvent::Deposited { amount: 200 });
    store.append(AccountEvent::Withdrawn { amount: 50 });

    // Take snapshot at index 2
    let snap = Snapshot {
        state: AccountState::from_events(store.events()),
        at_idx: store.len(),
    };

    // More events after snapshot
    store.append(AccountEvent::Deposited { amount: 75 });

    // Replay from snapshot
    let mut state = snap.state.clone();
    for event in store.events_from(snap.at_idx) {
        state.apply(event);
    }
    assert_eq!(state.balance, 225); // 200-50+75
}

#[test]
fn empty_log_gives_zero_balance() {
    let store = EventStore::new();
    let state = AccountState::from_events(store.events());
    assert_eq!(state.balance, 0);
}
