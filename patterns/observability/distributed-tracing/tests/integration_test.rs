use distributed_tracing::Tracer;

#[test]
fn trace_propagated_across_services() {
    let tracer = Tracer::new();
    let root = tracer.start_span("service-a", "handle_request", None);
    let trace_id = root.trace_id();
    let child = tracer.start_span("service-b", "process", Some(root.context()));
    let grandchild = tracer.start_span("service-c", "query_db", Some(child.context()));

    let gc_id = grandchild.span_id;
    let c_id = child.span_id;
    let r_id = root.span_id;

    tracer.finish_span(grandchild);
    tracer.finish_span(child);
    tracer.finish_span(root);

    let trace = tracer.get_trace(trace_id);
    assert_eq!(trace.spans.len(), 3);
    assert_eq!(trace.spans[0].service, "service-a");
    assert_eq!(trace.spans[1].parent_span_id, Some(r_id));
    assert_eq!(trace.spans[2].parent_span_id, Some(c_id));
    assert!(trace.spans.iter().all(|s| s.trace_id == trace_id));
}

#[test]
fn each_trace_has_unique_trace_id() {
    let tracer = Tracer::new();
    let s1 = tracer.start_span("svc", "op1", None);
    let t1 = s1.trace_id();
    tracer.finish_span(s1);

    let s2 = tracer.start_span("svc", "op2", None);
    let t2 = s2.trace_id();
    tracer.finish_span(s2);

    assert_ne!(t1, t2);
}
