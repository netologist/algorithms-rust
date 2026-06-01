/// Event Sourcing: state is derived from an append-only event log.
/// Demo domain: BankAccount with Deposit/Withdraw events.

#[derive(Debug, Clone)]
pub enum AccountEvent {
    Deposited { amount: i64 },
    Withdrawn { amount: i64 },
}

pub struct EventStore {
    events: Vec<AccountEvent>,
}

impl EventStore {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn append(&mut self, event: AccountEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[AccountEvent] {
        &self.events
    }

    /// Return events from index `from` onwards (for partial replay after snapshot).
    pub fn events_from(&self, from: usize) -> &[AccountEvent] {
        &self.events[from.min(self.events.len())..]
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AccountState {
    pub balance: i64,
}

impl AccountState {
    /// Rebuild state by replaying all events.
    pub fn from_events(events: &[AccountEvent]) -> Self {
        let mut state = AccountState::default();
        for event in events {
            state.apply(event);
        }
        state
    }

    /// Apply a single event.
    pub fn apply(&mut self, event: &AccountEvent) {
        match event {
            AccountEvent::Deposited { amount } => self.balance += amount,
            AccountEvent::Withdrawn { amount } => self.balance -= amount,
        }
    }
}

/// Snapshot: point-in-time state + event index at the time of snapshot.
pub struct Snapshot {
    pub state: AccountState,
    pub at_idx: usize,
}
