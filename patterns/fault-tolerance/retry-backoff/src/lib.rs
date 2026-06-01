use std::thread;
use std::time::Duration;

use rand::Rng;

/// Configuration for retry-with-backoff.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (not counting the first call).
    pub max_retries: u32,
    /// Initial delay before the first retry.
    pub base_delay: Duration,
    /// Maximum delay cap.
    pub max_delay: Duration,
    /// Add random jitter (±25%) to avoid thundering herd.
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            jitter: true,
        }
    }
}

impl RetryConfig {
    /// Compute the delay for attempt `n` (0-indexed).
    pub fn delay_for(&self, n: u32) -> Duration {
        let base = self.base_delay.as_millis() as f64;
        let exp = base * 2_f64.powi(n as i32);
        let capped = exp.min(self.max_delay.as_millis() as f64);
        let ms = if self.jitter {
            let factor = 1.0 + rand::thread_rng().gen_range(-0.25..=0.25);
            (capped * factor).max(1.0) as u64
        } else {
            capped as u64
        };
        Duration::from_millis(ms)
    }
}

/// Execute `f`, retrying up to `config.max_retries` times with exponential backoff.
pub fn retry<T, E, F>(config: RetryConfig, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut last_err;
    match f() {
        Ok(v) => return Ok(v),
        Err(e) => last_err = e,
    }
    for attempt in 0..config.max_retries {
        let delay = config.delay_for(attempt);
        thread::sleep(delay);
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}
