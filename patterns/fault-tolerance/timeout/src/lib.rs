use crossbeam_channel::bounded;
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum TimeoutError {
    /// The deadline elapsed before the operation completed.
    Elapsed,
}

/// Run `f` in a background thread. Return its result if it finishes within
/// `deadline`, otherwise return `Err(TimeoutError::Elapsed)`.
///
/// Note: the spawned thread cannot be killed — it runs to completion
/// regardless. Production code should use cancellation tokens.
pub fn with_timeout<T, F>(deadline: Duration, f: F) -> Result<T, TimeoutError>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = bounded(1);
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(deadline).map_err(|_| TimeoutError::Elapsed)
}
