use service_registry::ServiceRegistry;
use std::time::Duration;

fn main() {
    println!("=== Service Registry Demo ===\n");
    let mut reg = ServiceRegistry::new();

    println!("Registering 3 services...");
    reg.register("api", "api-1", "10.0.0.1:8080", Duration::from_secs(5));
    reg.register("api", "api-2", "10.0.0.2:8080", Duration::from_secs(5));
    reg.register("cache", "c-1", "10.0.0.3:6379", Duration::from_millis(100));

    println!(
        "Lookup 'api': {:?}",
        reg.lookup("api")
            .iter()
            .map(|i| &i.address)
            .collect::<Vec<_>>()
    );

    println!("\nWaiting for 'cache' TTL to expire (200ms)...");
    std::thread::sleep(Duration::from_millis(200));

    println!("Lookup 'cache' after TTL: {:?}", reg.lookup("cache"));

    println!("\nDeregistering api-2...");
    reg.deregister("api", "api-2");
    println!(
        "Lookup 'api': {:?}",
        reg.lookup("api")
            .iter()
            .map(|i| &i.address)
            .collect::<Vec<_>>()
    );
}
