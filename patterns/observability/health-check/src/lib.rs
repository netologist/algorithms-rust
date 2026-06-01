/// Health Check pattern: each service exposes a health endpoint.
/// The monitor aggregates check results into Healthy/Degraded/Unhealthy.

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

pub struct HealthCheck {
    pub name: String,
    pub check: Box<dyn Fn() -> HealthStatus + Send + Sync>,
}

impl HealthCheck {
    pub fn new(
        name: impl Into<String>,
        check: impl Fn() -> HealthStatus + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            check: Box::new(check),
        }
    }
}

pub struct HealthEndpoint {
    checks: Vec<HealthCheck>,
}

impl HealthEndpoint {
    pub fn new() -> Self {
        Self { checks: vec![] }
    }

    pub fn add_check(mut self, check: HealthCheck) -> Self {
        self.checks.push(check);
        self
    }

    /// Run all checks. Returns overall status and per-check results.
    ///
    /// Rules:
    /// - All pass → Healthy
    /// - Some pass, some fail → Degraded
    /// - All fail → Unhealthy
    pub fn evaluate(&self) -> (HealthStatus, Vec<(String, HealthStatus)>) {
        let results: Vec<(String, HealthStatus)> = self
            .checks
            .iter()
            .map(|c| (c.name.clone(), (c.check)()))
            .collect();

        let pass_count = results
            .iter()
            .filter(|(_, s)| *s == HealthStatus::Healthy)
            .count();
        let overall = if pass_count == results.len() {
            HealthStatus::Healthy
        } else if pass_count == 0 {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };

        (overall, results)
    }
}

impl Default for HealthEndpoint {
    fn default() -> Self {
        Self::new()
    }
}
