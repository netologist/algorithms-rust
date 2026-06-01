/// Saga (Orchestration): a central orchestrator drives each step in sequence.
/// On failure at step N, it runs compensations for steps 0..N in reverse.

#[derive(Debug, Clone, PartialEq)]
pub enum SagaResult {
    Committed,
    RolledBack { at_step: usize },
}

pub enum StepOutcome {
    Success,
    Failure,
}

pub struct SagaStep {
    pub name: String,
    pub execute: Box<dyn Fn() -> StepOutcome>,
    pub compensate: Box<dyn Fn()>,
}

impl SagaStep {
    pub fn new(
        name: impl Into<String>,
        execute: impl Fn() -> StepOutcome + 'static,
        compensate: impl Fn() + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            execute: Box::new(execute),
            compensate: Box::new(compensate),
        }
    }
}

pub struct SagaOrchestrator {
    steps: Vec<SagaStep>,
}

impl SagaOrchestrator {
    pub fn new() -> Self {
        Self { steps: vec![] }
    }

    pub fn add_step(mut self, step: SagaStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Execute steps in order. On failure, compensate in reverse.
    pub fn run(&self) -> SagaResult {
        let mut completed = vec![];

        for (i, step) in self.steps.iter().enumerate() {
            println!("  Step {}: {} ...", i + 1, step.name);
            match (step.execute)() {
                StepOutcome::Success => {
                    println!("    ✓ Success");
                    completed.push(i);
                }
                StepOutcome::Failure => {
                    println!("    ✗ Failed — rolling back completed steps");
                    for &j in completed.iter().rev() {
                        println!("    ↩ Compensating step {}: {}", j + 1, self.steps[j].name);
                        (self.steps[j].compensate)();
                    }
                    return SagaResult::RolledBack { at_step: i };
                }
            }
        }

        SagaResult::Committed
    }
}

impl Default for SagaOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
