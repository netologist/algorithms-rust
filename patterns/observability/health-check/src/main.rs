use health_check::{HealthCheck, HealthEndpoint, HealthStatus};

fn main() {
    println!("=== Health Check Demo ===\n");

    let scenarios = vec![
        ("All healthy", vec![true, true, true]),
        ("One failing", vec![true, false, true]),
        ("All failing", vec![false, false, false]),
    ];

    for (label, health) in scenarios {
        let ep = HealthEndpoint::new()
            .add_check(HealthCheck::new("database", {
                let h = health[0];
                move || {
                    if h {
                        HealthStatus::Healthy
                    } else {
                        HealthStatus::Unhealthy
                    }
                }
            }))
            .add_check(HealthCheck::new("cache", {
                let h = health[1];
                move || {
                    if h {
                        HealthStatus::Healthy
                    } else {
                        HealthStatus::Unhealthy
                    }
                }
            }))
            .add_check(HealthCheck::new("disk-space", {
                let h = health[2];
                move || {
                    if h {
                        HealthStatus::Healthy
                    } else {
                        HealthStatus::Unhealthy
                    }
                }
            }));

        let (status, results) = ep.evaluate();
        println!("Scenario: {} → {:?}", label, status);
        for (name, s) in &results {
            println!("  {} {:?}", name, s);
        }
        println!();
    }
}
