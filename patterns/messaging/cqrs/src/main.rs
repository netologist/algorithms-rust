use cqrs::{Command, CommandHandler, OrderProjection};

fn main() {
    println!("=== CQRS Demo ===\n");

    let mut handler = CommandHandler::new();
    let mut projection = OrderProjection::new();

    let cmds = vec![
        Command::CreateOrder {
            id: 1,
            item: "Laptop".into(),
        },
        Command::CreateOrder {
            id: 2,
            item: "Mouse".into(),
        },
        Command::CreateOrder {
            id: 3,
            item: "Keyboard".into(),
        },
        Command::CancelOrder { id: 2 },
    ];

    println!("Processing commands:");
    for cmd in cmds {
        println!("  CMD: {:?}", cmd);
        match handler.handle(cmd) {
            Ok(events) => {
                for e in &events {
                    println!("    EVENT: {:?}", e);
                    projection.apply(e);
                }
            }
            Err(e) => println!("    ERROR: {:?}", e),
        }
    }

    println!("\nRead model — active orders:");
    let mut active = projection.all_active();
    active.sort_by_key(|o| o.id);
    for order in active {
        println!("  #{}: {} [{:?}]", order.id, order.item, order.status);
    }
}
