/// Call `primary`. If it fails, call `fallback` and return its value instead.
pub fn with_fallback<T, E, P, F>(primary: P, fallback: F) -> T
where
    P: FnOnce() -> Result<T, E>,
    F: FnOnce() -> T,
{
    primary().unwrap_or_else(|_| fallback())
}

/// A cache-backed fallback — stores the last successful response and returns
/// it when the primary call fails.
pub struct FallbackCache<T: Clone> {
    cached: Option<T>,
}

impl<T: Clone> FallbackCache<T> {
    pub fn new() -> Self {
        Self { cached: None }
    }

    pub fn cached(&self) -> Option<&T> {
        self.cached.as_ref()
    }

    /// Call `f`. On success, cache and return the value.
    /// On failure, return the last cached value if available, else `Err(CacheEmpty)`.
    pub fn call<E, F>(&mut self, f: F) -> Result<T, CacheEmpty>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match f() {
            Ok(v) => {
                self.cached = Some(v.clone());
                Ok(v)
            }
            Err(_) => self.cached.clone().ok_or(CacheEmpty),
        }
    }
}

impl<T: Clone> Default for FallbackCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct CacheEmpty;

impl std::fmt::Display for CacheEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no cached value available")
    }
}
