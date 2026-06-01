/// CQRS: Command Query Responsibility Segregation.
///
/// Write side: Commands → Events (via CommandHandler)
/// Read side:  Events → Projections (via OrderProjection)
use std::collections::HashMap;

// --- Write Side ---

#[derive(Debug, Clone)]
pub enum Command {
    CreateOrder { id: u64, item: String },
    CancelOrder { id: u64 },
}

#[derive(Debug, Clone)]
pub enum DomainEvent {
    OrderCreated { id: u64, item: String },
    OrderCancelled { id: u64 },
}

#[derive(Debug)]
pub enum CqrsError {
    OrderAlreadyExists,
    OrderNotFound,
}

pub struct CommandHandler {
    event_store: Vec<DomainEvent>,
    order_ids: std::collections::HashSet<u64>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            event_store: vec![],
            order_ids: std::collections::HashSet::new(),
        }
    }

    pub fn handle(&mut self, cmd: Command) -> Result<Vec<DomainEvent>, CqrsError> {
        match cmd {
            Command::CreateOrder { id, item } => {
                if self.order_ids.contains(&id) {
                    return Err(CqrsError::OrderAlreadyExists);
                }
                self.order_ids.insert(id);
                let event = DomainEvent::OrderCreated { id, item };
                self.event_store.push(event.clone());
                Ok(vec![event])
            }
            Command::CancelOrder { id } => {
                if !self.order_ids.contains(&id) {
                    return Err(CqrsError::OrderNotFound);
                }
                let event = DomainEvent::OrderCancelled { id };
                self.event_store.push(event.clone());
                Ok(vec![event])
            }
        }
    }

    pub fn events(&self) -> &[DomainEvent] {
        &self.event_store
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

// --- Read Side ---

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    Active,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct OrderView {
    pub id: u64,
    pub item: String,
    pub status: OrderStatus,
}

pub struct OrderProjection {
    orders: HashMap<u64, OrderView>,
}

impl OrderProjection {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    pub fn apply(&mut self, event: &DomainEvent) {
        match event {
            DomainEvent::OrderCreated { id, item } => {
                self.orders.insert(
                    *id,
                    OrderView {
                        id: *id,
                        item: item.clone(),
                        status: OrderStatus::Active,
                    },
                );
            }
            DomainEvent::OrderCancelled { id } => {
                if let Some(o) = self.orders.get_mut(id) {
                    o.status = OrderStatus::Cancelled;
                }
            }
        }
    }

    pub fn get(&self, id: u64) -> Option<&OrderView> {
        self.orders.get(&id)
    }

    pub fn all_active(&self) -> Vec<&OrderView> {
        self.orders
            .values()
            .filter(|o| o.status == OrderStatus::Active)
            .collect()
    }
}

impl Default for OrderProjection {
    fn default() -> Self {
        Self::new()
    }
}
