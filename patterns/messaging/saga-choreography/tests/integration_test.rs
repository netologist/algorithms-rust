use saga_choreography::{ChoreographySaga, EventBus, SagaEvent};

#[test]
fn happy_path_completes_order() {
    let saga = ChoreographySaga::new(false);
    let mut bus = EventBus::new();
    bus.publish(SagaEvent::OrderPlaced { order_id: 1 });
    saga.run(&mut bus);

    assert!(bus
        .history
        .contains(&SagaEvent::OrderCompleted { order_id: 1 }));
    assert!(!bus
        .history
        .contains(&SagaEvent::OrderCancelled { order_id: 1 }));
}

#[test]
fn payment_failure_cancels_order() {
    let saga = ChoreographySaga::new(true);
    let mut bus = EventBus::new();
    bus.publish(SagaEvent::OrderPlaced { order_id: 2 });
    saga.run(&mut bus);

    assert!(bus
        .history
        .contains(&SagaEvent::OrderCancelled { order_id: 2 }));
    assert!(!bus
        .history
        .contains(&SagaEvent::OrderCompleted { order_id: 2 }));
}
