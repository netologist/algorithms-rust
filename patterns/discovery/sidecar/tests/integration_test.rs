use sidecar::{AuthInterceptor, Interceptor, LoggingInterceptor, Request, Sidecar};

#[test]
fn auth_interceptor_adds_authorization_header() {
    let interceptor = AuthInterceptor {
        token: "abc".into(),
    };
    let mut req = Request::new("svc", "payload");
    interceptor.on_request(&mut req);
    assert_eq!(req.headers.get("Authorization").unwrap(), "Bearer abc");
}

#[test]
fn proxy_returns_response_from_target() {
    let proxy = Sidecar::new().add_interceptor(Box::new(LoggingInterceptor));
    let req = Request::new("my-service", "hello");
    let resp = proxy.proxy(req);
    assert!(resp.body.contains("my-service"));
}

#[test]
fn multiple_interceptors_all_run() {
    let proxy = Sidecar::new()
        .add_interceptor(Box::new(AuthInterceptor {
            token: "tok".into(),
        }))
        .add_interceptor(Box::new(LoggingInterceptor));
    let req = Request::new("svc", "data");
    let resp = proxy.proxy(req);
    assert!(!resp.body.is_empty());
}
