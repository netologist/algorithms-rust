use std::collections::HashMap;

pub struct Request {
    pub target: String,
    pub payload: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(target: &str, payload: &str) -> Self {
        Self {
            target: target.into(),
            payload: payload.into(),
            headers: HashMap::new(),
        }
    }
}

pub struct Response {
    pub body: String,
    pub headers: HashMap<String, String>,
}

pub trait Interceptor: Send + Sync {
    fn on_request(&self, req: &mut Request);
    fn on_response(&self, resp: &mut Response);
}

/// Adds an Authorization header to every outbound request.
pub struct AuthInterceptor {
    pub token: String,
}

impl Interceptor for AuthInterceptor {
    fn on_request(&self, req: &mut Request) {
        req.headers
            .insert("Authorization".into(), format!("Bearer {}", self.token));
    }
    fn on_response(&self, _resp: &mut Response) {}
}

/// Logs request and response details.
pub struct LoggingInterceptor;

impl Interceptor for LoggingInterceptor {
    fn on_request(&self, req: &mut Request) {
        println!("  [sidecar] → {} | payload: {}", req.target, req.payload);
    }
    fn on_response(&self, resp: &mut Response) {
        println!("  [sidecar] ← body: {}", resp.body);
    }
}

/// Sidecar proxy: intercepts all outbound calls, runs interceptors,
/// then simulates the actual service call.
pub struct Sidecar {
    interceptors: Vec<Box<dyn Interceptor>>,
}

impl Sidecar {
    pub fn new() -> Self {
        Self {
            interceptors: vec![],
        }
    }

    pub fn add_interceptor(mut self, i: Box<dyn Interceptor>) -> Self {
        self.interceptors.push(i);
        self
    }

    /// Proxy a request: run on_request interceptors, simulate call, run on_response interceptors.
    pub fn proxy(&self, mut req: Request) -> Response {
        for i in &self.interceptors {
            i.on_request(&mut req);
        }
        // Simulate the actual service call
        let mut resp = Response {
            body: format!("OK from {}", req.target),
            headers: HashMap::new(),
        };
        for i in &self.interceptors {
            i.on_response(&mut resp);
        }
        resp
    }
}

impl Default for Sidecar {
    fn default() -> Self {
        Self::new()
    }
}
