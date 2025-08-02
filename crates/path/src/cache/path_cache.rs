use std::collections::{HashMap, VecDeque};

use super::{CacheKey, EvictionPolicy};
use crate::Point;

/// Stores previously computed paths with an eviction policy.
#[derive(Debug)]
pub struct PathCache {
    map: HashMap<CacheKey, Vec<Point>>,
    order: VecDeque<CacheKey>,
    max_size: usize,
    policy: EvictionPolicy,
    hits: u64,
    misses: u64,
}

impl PathCache {
    /// Creates a new cache with the given `max_size` and `policy`.
    pub fn new(max_size: usize, policy: EvictionPolicy) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            max_size,
            policy,
            hits: 0,
            misses: 0,
        }
    }

    /// Attempts to retrieve a path from the cache.
    pub fn get(&mut self, key: &CacheKey) -> Option<&Vec<Point>> {
        if let Some(path) = self.map.get(key) {
            self.hits += 1;
            if self.policy == EvictionPolicy::Lru {
                if let Some(pos) = self.order.iter().position(|k| k == key) {
                    self.order.remove(pos);
                    self.order.push_front(*key);
                }
            }
            Some(path)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Inserts a new path into the cache.
    pub fn insert(&mut self, key: CacheKey, path: Vec<Point>) {
        if self.map.contains_key(&key) {
            if let Some(pos) = self.order.iter().position(|k| k == &key) {
                self.order.remove(pos);
            }
        } else if self.map.len() == self.max_size {
            if let Some(old_key) = self.order.pop_back() {
                self.map.remove(&old_key);
            }
        }
        self.order.push_front(key);
        self.map.insert(key, path);
    }

    /// Number of cache hits.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Number of cache misses.
    pub fn misses(&self) -> u64 {
        self.misses
    }
}
