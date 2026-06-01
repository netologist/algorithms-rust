use saga_choreography::{ChoreographySaga, EventBus, SagaEvent};

fn main() {
    println!("=== Saga Choreography Demo ===\n");

    println!("--- Happy path (payment succeeds) ---");
    let saga = ChoreographySaga::new(false);
    let mut bus = EventBus::new();
    bus.publish(SagaEvent::OrderPlaced { order_id: 42 });
    saga.run(&mut bus);
    println!("  Final: OrderCompleted\n");

    println!("--- Failure path (payment fails) ---");
    let saga = ChoreographySaga::new(true);
    let mut bus = EventBus::new();
    bus.publish(SagaEvent::OrderPlaced { order_id: 43 });
    saga.run(&mut bus);
    println!("  Final: OrderCancelled (compensation applied)");
}
