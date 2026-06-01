use std::time::{Duration, Instant};

/// The three states of a circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Requests pass through normally.
    Closed,
    /// Requests are rejected immediately.
    Open,
    /// One probe request is allowed through.
    HalfOpen,
}

/// Configuration for the circuit breaker.
pub struct Config {
    /// Number of consecutive failures before opening.
    pub failure_threshold: u32,
    /// Number of consecutive successes in HalfOpen to close.
    pub success_threshold: u32,
    /// How long to wait in Open before moving to HalfOpen.
    pub half_open_timeout: Duration,
}

pub struct CircuitBreaker {
    config: Config,
    state: CircuitState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }

    /// Returns the current state, transitioning Open→HalfOpen if the timeout has elapsed.
    pub fn state(&mut self) -> CircuitState {
        if self.state == CircuitState::Open {
            if let Some(t) = self.last_failure_time {
                if t.elapsed() >= self.config.half_open_timeout {
                    self.state = CircuitState::HalfOpen;
                    self.success_count = 0;
                }
            }
        }
        self.state
    }

    /// Record a failure outcome.
    pub fn record_failure(&mut self) {
        self.last_failure_time = Some(Instant::now());
        match self.state {
            CircuitState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.config.failure_threshold {
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.success_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Record a success outcome.
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.config.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                }
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Execute `f`. Returns Err if the circuit is Open; otherwise runs `f` and
    /// records the outcome automatically.
    pub fn call<T, E, F>(&mut self, f: F) -> Result<T, CircuitError<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if self.state() == CircuitState::Open {
            return Err(CircuitError::Open);
        }
        match f() {
            Ok(v) => {
                self.record_success();
                Ok(v)
            }
            Err(e) => {
                self.record_failure();
                Err(CircuitError::Inner(e))
            }
        }
    }
}

/// Error type returned by `CircuitBreaker::call`.
#[derive(Debug)]
pub enum CircuitError<E> {
    /// The circuit was open — request was not attempted.
    Open,
    /// The request was attempted but returned this error.
    Inner(E),
}
