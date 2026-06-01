use service_registry::ServiceRegistry;
use std::time::Duration;

#[test]
fn register_and_lookup() {
    let mut reg = ServiceRegistry::new();
    reg.register("users", "u1", "10.0.0.1:8080", Duration::from_secs(60));
    let instances = reg.lookup("users");
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].address, "10.0.0.1:8080");
}

#[test]
fn deregister_removes_instance() {
    let mut reg = ServiceRegistry::new();
    reg.register("svc", "i1", "1.2.3.4:80", Duration::from_secs(60));
    reg.deregister("svc", "i1");
    assert!(reg.lookup("svc").is_empty());
}

#[test]
fn expired_ttl_evicted_on_lookup() {
    let mut reg = ServiceRegistry::new();
    reg.register("svc", "i1", "addr", Duration::from_millis(1));
    std::thread::sleep(Duration::from_millis(10));
    assert!(reg.lookup("svc").is_empty());
}

#[test]
fn unknown_service_returns_empty() {
    let mut reg = ServiceRegistry::new();
    assert!(reg.lookup("unknown").is_empty());
}
