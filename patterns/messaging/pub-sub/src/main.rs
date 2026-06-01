use pub_sub::Broker;
use std::time::Duration;

fn main() {
    println!("=== Pub-Sub Broker Demo ===\n");

    let mut broker = Broker::new();

    let news_sub1 = broker.subscribe("news");
    let news_sub2 = broker.subscribe("news");
    let alerts_sub = broker.subscribe("alerts");

    println!("Publishing to 'news'...");
    broker.publish("news", "Rust 2.0 announced!");
    broker.publish("news", "New async features landed");

    println!("Publishing to 'alerts'...");
    broker.publish("alerts", "System CPU > 90%");

    println!("\nSubscriber 1 (news):");
    while let Ok(msg) = news_sub1.recv_timeout(Duration::from_millis(50)) {
        println!("  → {}", msg);
    }

    println!("Subscriber 2 (news):");
    while let Ok(msg) = news_sub2.recv_timeout(Duration::from_millis(50)) {
        println!("  → {}", msg);
    }

    println!("Subscriber (alerts):");
    while let Ok(msg) = alerts_sub.recv_timeout(Duration::from_millis(50)) {
        println!("  → {}", msg);
    }

    println!("\nLate subscriber joins 'news'...");
    let late_sub = broker.subscribe("news");
    broker.publish("news", "New article (after late join)");
    println!(
        "  Late sub sees only: {:?}",
        late_sub.recv_timeout(Duration::from_millis(50)).ok()
    );
}
