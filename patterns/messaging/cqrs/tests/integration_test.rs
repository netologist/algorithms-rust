use cqrs::{Command, CommandHandler, OrderProjection, OrderStatus};

#[test]
fn create_order_produces_event_and_projection() {
    let mut handler = CommandHandler::new();
    let mut projection = OrderProjection::new();

    let events = handler
        .handle(Command::CreateOrder {
            id: 1,
            item: "Widget".into(),
        })
        .unwrap();
    for e in &events {
        projection.apply(e);
    }

    let order = projection.get(1).unwrap();
    assert_eq!(order.item, "Widget");
    assert_eq!(order.status, OrderStatus::Active);
}

#[test]
fn cancel_order_updates_projection() {
    let mut handler = CommandHandler::new();
    let mut projection = OrderProjection::new();

    for e in handler
        .handle(Command::CreateOrder {
            id: 1,
            item: "Book".into(),
        })
        .unwrap()
    {
        projection.apply(&e);
    }
    for e in handler.handle(Command::CancelOrder { id: 1 }).unwrap() {
        projection.apply(&e);
    }

    assert_eq!(projection.get(1).unwrap().status, OrderStatus::Cancelled);
    assert!(projection.all_active().is_empty());
}

#[test]
fn duplicate_create_returns_error() {
    let mut handler = CommandHandler::new();
    handler
        .handle(Command::CreateOrder {
            id: 1,
            item: "A".into(),
        })
        .unwrap();
    assert!(handler
        .handle(Command::CreateOrder {
            id: 1,
            item: "B".into()
        })
        .is_err());
}
