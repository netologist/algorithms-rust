use health_check::{HealthCheck, HealthEndpoint, HealthStatus};

#[test]
fn all_pass_returns_healthy() {
    let ep = HealthEndpoint::new()
        .add_check(HealthCheck::new("db", || HealthStatus::Healthy))
        .add_check(HealthCheck::new("cache", || HealthStatus::Healthy));
    let (status, _) = ep.evaluate();
    assert_eq!(status, HealthStatus::Healthy);
}

#[test]
fn one_fail_returns_degraded() {
    let ep = HealthEndpoint::new()
        .add_check(HealthCheck::new("db", || HealthStatus::Healthy))
        .add_check(HealthCheck::new("cache", || HealthStatus::Unhealthy));
    let (status, _) = ep.evaluate();
    assert_eq!(status, HealthStatus::Degraded);
}

#[test]
fn all_fail_returns_unhealthy() {
    let ep = HealthEndpoint::new()
        .add_check(HealthCheck::new("db", || HealthStatus::Unhealthy))
        .add_check(HealthCheck::new("cache", || HealthStatus::Unhealthy));
    let (status, _) = ep.evaluate();
    assert_eq!(status, HealthStatus::Unhealthy);
}

#[test]
fn results_contain_per_check_status() {
    let ep = HealthEndpoint::new().add_check(HealthCheck::new("db", || HealthStatus::Healthy));
    let (_, results) = ep.evaluate();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "db");
}
