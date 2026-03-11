use lru_cache::LruCache;

#[test]
fn test_basic_get_put() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(&1), Some(10));
    assert_eq!(cache.get(&2), Some(20));
    assert_eq!(cache.get(&3), None);
}

#[test]
fn test_eviction() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(3, 30); // evicts 1
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(20));
    assert_eq!(cache.get(&3), Some(30));
}

#[test]
fn test_access_updates_recency() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    // Access 1 to make it most-recent; 2 becomes LRU.
    cache.get(&1);
    cache.put(3, 30); // evicts 2
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&1), Some(10));
    assert_eq!(cache.get(&3), Some(30));
}

#[test]
fn test_overwrite_existing_key() {
    let mut cache = LruCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    cache.put(1, 100); // update value, 1 becomes most-recent
    cache.put(3, 30); // evicts 2
    assert_eq!(cache.get(&1), Some(100));
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.get(&3), Some(30));
}

#[test]
fn test_capacity_one() {
    let mut cache = LruCache::new(1);
    cache.put(1, 10);
    assert_eq!(cache.get(&1), Some(10));
    cache.put(2, 20); // evicts 1
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(20));
}

#[test]
fn test_len_and_is_empty() {
    let mut cache = LruCache::new(3);
    assert!(cache.is_empty());
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.len(), 2);
    cache.put(3, 30);
    cache.put(4, 40); // evicts 1
    assert_eq!(cache.len(), 3);
}

#[test]
#[should_panic(expected = "capacity must be > 0")]
fn test_zero_capacity_panics() {
    let _cache = LruCache::new(0);
}
