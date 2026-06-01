use fallback::{with_fallback, FallbackCache};

fn main() {
    println!("=== Fallback Pattern Demo ===\n");

    println!("--- Simple fallback ---");
    let r = with_fallback(
        || Ok::<&str, &str>("live data from API"),
        || "default value",
    );
    println!("Primary succeeds: {:?}", r);

    let r = with_fallback(
        || Err::<&str, &str>("connection refused"),
        || "default value",
    );
    println!("Primary fails:    {:?}\n", r);

    println!("--- Cache-backed fallback ---");
    let mut cache: FallbackCache<String> = FallbackCache::new();

    println!("Call 1 (primary OK)...");
    let r = cache.call(|| Ok::<String, &str>("user_data_v1".into()));
    println!("  Result: {:?}", r);

    println!("Call 2 (primary fails, cache hit)...");
    let r = cache.call(|| Err::<String, &str>("service down"));
    println!("  Result: {:?} (served from cache)", r);

    println!("Call 3 (primary OK, cache updated)...");
    let r = cache.call(|| Ok::<String, &str>("user_data_v2".into()));
    println!("  Result: {:?}", r);

    println!("Call 4 (primary fails again, newer cache)...");
    let r = cache.call(|| Err::<String, &str>("service down"));
    println!("  Result: {:?}", r);
}
