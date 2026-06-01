use saga_orchestration::{SagaOrchestrator, SagaStep, StepOutcome};

fn main() {
    println!("=== Saga Orchestration Demo ===\n");

    println!("--- Happy path (all steps succeed) ---");
    let saga = SagaOrchestrator::new()
        .add_step(SagaStep::new(
            "Reserve Inventory",
            || {
                println!("    → Inventory reserved");
                StepOutcome::Success
            },
            || {
                println!("    ↩ Inventory released");
            },
        ))
        .add_step(SagaStep::new(
            "Charge Payment",
            || {
                println!("    → Payment charged");
                StepOutcome::Success
            },
            || {
                println!("    ↩ Payment refunded");
            },
        ))
        .add_step(SagaStep::new(
            "Send Confirmation",
            || {
                println!("    → Email sent");
                StepOutcome::Success
            },
            || {},
        ));
    println!("Result: {:?}\n", saga.run());

    println!("--- Failure at step 2 (payment fails) ---");
    let saga = SagaOrchestrator::new()
        .add_step(SagaStep::new(
            "Reserve Inventory",
            || {
                println!("    → Inventory reserved");
                StepOutcome::Success
            },
            || {
                println!("    ↩ Inventory released");
            },
        ))
        .add_step(SagaStep::new(
            "Charge Payment",
            || {
                println!("    → Payment FAILED");
                StepOutcome::Failure
            },
            || {
                println!("    ↩ Payment refunded");
            },
        ))
        .add_step(SagaStep::new(
            "Send Confirmation",
            || StepOutcome::Success,
            || {},
        ));
    println!("Result: {:?}", saga.run());
}
