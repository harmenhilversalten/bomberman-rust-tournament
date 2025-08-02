use path::Point;
use path::cache::{CacheKey, EvictionPolicy, PathCache};

#[test]
fn cache_records_hits_and_misses() {
    let mut cache = PathCache::new(2, EvictionPolicy::Lru);
    let key = CacheKey::new(Point::new(0, 0), Point::new(1, 1));
    assert!(cache.get(&key).is_none());
    assert_eq!(cache.misses(), 1);

    cache.insert(key, vec![Point::new(0, 0), Point::new(1, 1)]);
    assert!(cache.get(&key).is_some());
    assert_eq!(cache.hits(), 1);
}

#[test]
fn lru_policy_evicts_least_recently_used() {
    let mut cache = PathCache::new(1, EvictionPolicy::Lru);
    let k1 = CacheKey::new(Point::new(0, 0), Point::new(1, 0));
    let k2 = CacheKey::new(Point::new(0, 1), Point::new(1, 1));
    cache.insert(k1, vec![Point::new(0, 0)]);
    cache.insert(k2, vec![Point::new(0, 1)]);
    assert!(cache.get(&k1).is_none());
    assert!(cache.get(&k2).is_some());
}

#[test]
fn fifo_policy_evicts_in_insertion_order() {
    let mut cache = PathCache::new(1, EvictionPolicy::Fifo);
    let k1 = CacheKey::new(Point::new(0, 0), Point::new(1, 0));
    let k2 = CacheKey::new(Point::new(0, 1), Point::new(1, 1));
    cache.insert(k1, vec![Point::new(0, 0)]);
    // Access k1 to ensure FIFO ignores usage
    let _ = cache.get(&k1);
    cache.insert(k2, vec![Point::new(0, 1)]);
    assert!(cache.get(&k1).is_none());
    assert!(cache.get(&k2).is_some());
}
