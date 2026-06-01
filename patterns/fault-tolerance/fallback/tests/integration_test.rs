use fallback::{with_fallback, FallbackCache};

#[test]
fn returns_primary_on_success() {
    let result = with_fallback(|| Ok::<&str, &str>("primary"), || "cached");
    assert_eq!(result, "primary");
}

#[test]
fn returns_fallback_on_failure() {
    let result = with_fallback(|| Err::<&str, &str>("error"), || "cached");
    assert_eq!(result, "cached");
}

#[test]
fn cache_stores_last_success() {
    let mut cache: FallbackCache<String> = FallbackCache::new();
    cache.call(|| Ok::<String, &str>("v1".into())).unwrap();
    let result = cache.call(|| Err::<String, &str>("down"));
    assert_eq!(result.unwrap(), "v1");
}

#[test]
fn cache_returns_err_when_empty_and_primary_fails() {
    let mut cache: FallbackCache<String> = FallbackCache::new();
    let result = cache.call(|| Err::<String, &str>("down"));
    assert!(result.is_err());
}

#[test]
fn cache_updates_on_new_success() {
    let mut cache: FallbackCache<String> = FallbackCache::new();
    cache.call(|| Ok::<String, &str>("old".into())).unwrap();
    cache.call(|| Ok::<String, &str>("new".into())).unwrap();
    let result = cache.call(|| Err::<String, &str>("down"));
    assert_eq!(result.unwrap(), "new");
}
