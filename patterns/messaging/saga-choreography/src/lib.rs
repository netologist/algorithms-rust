/// Saga (Choreography): each service reacts to domain events and publishes its own.
/// No central coordinator — services communicate only through events.
///
/// Order flow:
///   OrderPlaced → PaymentService → PaymentProcessed/Failed
///   PaymentProcessed → InventoryService → InventoryReserved
///   InventoryReserved → OrderService → OrderCompleted
///
/// Compensation:
///   PaymentFailed → OrderService → OrderCancelled
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum SagaEvent {
    OrderPlaced { order_id: u64 },
    PaymentProcessed { order_id: u64 },
    PaymentFailed { order_id: u64 },
    InventoryReserved { order_id: u64 },
    InventoryReleased { order_id: u64 },
    OrderCompleted { order_id: u64 },
    OrderCancelled { order_id: u64 },
}

pub struct EventBus {
    queue: VecDeque<SagaEvent>,
    pub history: Vec<SagaEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            history: vec![],
        }
    }

    pub fn publish(&mut self, event: SagaEvent) {
        self.history.push(event.clone());
        self.queue.push_back(event);
    }

    pub fn next(&mut self) -> Option<SagaEvent> {
        self.queue.pop_front()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChoreographySaga {
    /// If true, payment fails (triggers compensating transactions)
    pub payment_fails: bool,
}

impl ChoreographySaga {
    pub fn new(payment_fails: bool) -> Self {
        Self { payment_fails }
    }

    /// Process all events on the bus until the queue is empty.
    pub fn run(&self, bus: &mut EventBus) {
        while let Some(event) = bus.next() {
            match event {
                SagaEvent::OrderPlaced { order_id } => {
                    println!("  PaymentService ← OrderPlaced({})", order_id);
                    if self.payment_fails {
                        println!("  PaymentService → PaymentFailed({})", order_id);
                        bus.publish(SagaEvent::PaymentFailed { order_id });
                    } else {
                        println!("  PaymentService → PaymentProcessed({})", order_id);
                        bus.publish(SagaEvent::PaymentProcessed { order_id });
                    }
                }
                SagaEvent::PaymentProcessed { order_id } => {
                    println!("  InventoryService ← PaymentProcessed({})", order_id);
                    println!("  InventoryService → InventoryReserved({})", order_id);
                    bus.publish(SagaEvent::InventoryReserved { order_id });
                }
                SagaEvent::PaymentFailed { order_id } => {
                    println!("  OrderService ← PaymentFailed({}) [compensate]", order_id);
                    println!("  OrderService → OrderCancelled({})", order_id);
                    bus.publish(SagaEvent::OrderCancelled { order_id });
                }
                SagaEvent::InventoryReserved { order_id } => {
                    println!("  OrderService ← InventoryReserved({})", order_id);
                    println!("  OrderService → OrderCompleted({})", order_id);
                    bus.publish(SagaEvent::OrderCompleted { order_id });
                }
                _ => {} // terminal events
            }
        }
    }
}
