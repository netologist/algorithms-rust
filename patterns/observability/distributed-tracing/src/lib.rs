use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub type TraceId = u64;
pub type SpanId = u64;

#[derive(Debug, Clone)]
pub struct SpanContext {
    pub trace_id: TraceId,
    pub span_id: SpanId,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub span_id: SpanId,
    pub trace_id: TraceId,
    pub parent_span_id: Option<SpanId>,
    pub service: String,
    pub operation: String,
    pub start: Instant,
    pub duration_us: Option<u64>,
}

impl Span {
    pub fn context(&self) -> SpanContext {
        SpanContext {
            trace_id: self.trace_id,
            span_id: self.span_id,
        }
    }

    pub fn trace_id(&self) -> TraceId {
        self.trace_id
    }
}

#[derive(Debug)]
pub struct Trace {
    pub spans: Vec<Span>,
}

pub struct Tracer {
    spans: Arc<Mutex<HashMap<SpanId, Span>>>,
    next_span: Arc<Mutex<SpanId>>,
    next_trace: Arc<Mutex<TraceId>>,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            spans: Arc::new(Mutex::new(HashMap::new())),
            next_span: Arc::new(Mutex::new(1)),
            next_trace: Arc::new(Mutex::new(1)),
        }
    }

    fn next_span_id(&self) -> SpanId {
        let mut n = self.next_span.lock().unwrap();
        let id = *n;
        *n += 1;
        id
    }

    fn next_trace_id(&self) -> TraceId {
        let mut n = self.next_trace.lock().unwrap();
        let id = *n;
        *n += 1;
        id
    }

    pub fn start_span(&self, service: &str, operation: &str, parent: Option<SpanContext>) -> Span {
        let span_id = self.next_span_id();
        let trace_id = parent
            .as_ref()
            .map(|p| p.trace_id)
            .unwrap_or_else(|| self.next_trace_id());
        Span {
            span_id,
            trace_id,
            parent_span_id: parent.map(|p| p.span_id),
            service: service.into(),
            operation: operation.into(),
            start: Instant::now(),
            duration_us: None,
        }
    }

    pub fn finish_span(&self, mut span: Span) {
        span.duration_us = Some(span.start.elapsed().as_micros() as u64);
        self.spans.lock().unwrap().insert(span.span_id, span);
    }

    pub fn get_trace(&self, trace_id: TraceId) -> Trace {
        let spans = self.spans.lock().unwrap();
        let mut v: Vec<Span> = spans
            .values()
            .filter(|s| s.trace_id == trace_id)
            .cloned()
            .collect();
        v.sort_by_key(|s| s.span_id);
        Trace { spans: v }
    }
}

impl Default for Tracer {
    fn default() -> Self {
        Self::new()
    }
}

/// Pretty-print a trace as an indented span tree.
pub fn print_trace(trace: &Trace) {
    fn indent(span: &Span, all: &[Span]) -> usize {
        match span.parent_span_id {
            None => 0,
            Some(p) => 1 + indent(all.iter().find(|s| s.span_id == p).unwrap(), all),
        }
    }
    for span in &trace.spans {
        let d = span.duration_us.unwrap_or(0);
        println!(
            "{}{} → {} [{}μs]",
            "  ".repeat(indent(span, &trace.spans)),
            span.service,
            span.operation,
            d,
        );
    }
}
