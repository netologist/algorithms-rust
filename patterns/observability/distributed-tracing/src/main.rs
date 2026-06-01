use distributed_tracing::{print_trace, Tracer};
use std::time::Duration;

fn main() {
    println!("=== Distributed Tracing Demo ===\n");

    let tracer = Tracer::new();

    // Simulate: API gateway → order service → inventory service + payment service
    let root = tracer.start_span("api-gateway", "POST /orders", None);
    let ctx_root = root.context();
    std::thread::sleep(Duration::from_micros(100));

    let order_span = tracer.start_span("order-service", "create_order", Some(ctx_root));
    let ctx_order = order_span.context();
    std::thread::sleep(Duration::from_micros(200));

    let inv_span = tracer.start_span(
        "inventory-service",
        "reserve_stock",
        Some(ctx_order.clone()),
    );
    std::thread::sleep(Duration::from_micros(150));
    tracer.finish_span(inv_span);

    let pay_span = tracer.start_span("payment-service", "charge_card", Some(ctx_order));
    std::thread::sleep(Duration::from_micros(300));
    tracer.finish_span(pay_span);

    tracer.finish_span(order_span);
    tracer.finish_span(root.clone());

    let trace = tracer.get_trace(root.trace_id());
    println!("Trace ID: {}", root.trace_id());
    println!("Span tree:\n");
    print_trace(&trace);
    println!("\nTotal spans: {}", trace.spans.len());
}
