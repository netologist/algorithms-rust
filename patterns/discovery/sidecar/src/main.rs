use sidecar::{AuthInterceptor, LoggingInterceptor, Request, Sidecar};

fn main() {
    println!("=== Sidecar Pattern Demo ===\n");

    let proxy = Sidecar::new()
        .add_interceptor(Box::new(LoggingInterceptor))
        .add_interceptor(Box::new(AuthInterceptor {
            token: "<demo-token>".into(),
        }));

    println!("Making 3 outbound calls through sidecar:\n");
    for svc in &["user-service", "payment-service", "inventory-service"] {
        let req = Request::new(&format!("http://{}/api", svc), r#"{"action":"get"}"#);
        let resp = proxy.proxy(req);
        println!("  Response: {}\n", resp.body);
    }
}
