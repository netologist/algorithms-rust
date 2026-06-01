use saga_orchestration::{SagaOrchestrator, SagaResult, SagaStep, StepOutcome};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn all_steps_succeed_committed() {
    let saga = SagaOrchestrator::new()
        .add_step(SagaStep::new("step1", || StepOutcome::Success, || {}))
        .add_step(SagaStep::new("step2", || StepOutcome::Success, || {}))
        .add_step(SagaStep::new("step3", || StepOutcome::Success, || {}));
    assert_eq!(saga.run(), SagaResult::Committed);
}

#[test]
fn failure_at_step_2_compensates_step_1() {
    let compensated = Arc::new(AtomicBool::new(false));
    let c = compensated.clone();

    let saga = SagaOrchestrator::new()
        .add_step(SagaStep::new(
            "step1",
            || StepOutcome::Success,
            move || {
                c.store(true, Ordering::SeqCst);
            },
        ))
        .add_step(SagaStep::new("step2", || StepOutcome::Failure, || {}));

    let result = saga.run();
    assert_eq!(result, SagaResult::RolledBack { at_step: 1 });
    assert!(
        compensated.load(Ordering::SeqCst),
        "step1 should have been compensated"
    );
}
