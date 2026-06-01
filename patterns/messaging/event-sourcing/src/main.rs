use event_sourcing::{AccountEvent, AccountState, EventStore, Snapshot};

fn main() {
    println!("=== Event Sourcing Demo ===\n");

    let mut store = EventStore::new();
    let actions = vec![
        AccountEvent::Deposited { amount: 1000 },
        AccountEvent::Withdrawn { amount: 200 },
        AccountEvent::Deposited { amount: 500 },
        AccountEvent::Withdrawn { amount: 100 },
    ];

    println!("Appending events:");
    for event in actions {
        println!("  {:?}", event);
        store.append(event);
    }

    let state = AccountState::from_events(store.events());
    println!("\nRebuilt state: balance = {}", state.balance);

    println!("\nTaking snapshot at event #{}", store.len());
    let snap = Snapshot {
        state: state.clone(),
        at_idx: store.len(),
    };

    println!("Appending 2 more events after snapshot...");
    store.append(AccountEvent::Deposited { amount: 300 });
    store.append(AccountEvent::Withdrawn { amount: 50 });

    println!("Replaying from snapshot...");
    let mut s = snap.state.clone();
    for e in store.events_from(snap.at_idx) {
        s.apply(e);
    }
    println!("Final balance: {}", s.balance);
}
