//! <https://leetcode.com/problems/lru-cache/>
//! Medium - [hash-table, linked-list, design, doubly-linked-list]
//!
//! Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
//! Implement the LRUCache class:
//! - LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
//! - int get(int key) Return the value of the key if the key exists, otherwise return -1.
//! - void put(int key, int value) Update the value of the key if the key exists.
//!   Otherwise, add the key-value pair to the cache.
//!   If the number of keys exceeds the capacity from this operation,
//!   evict the least recently used key.
//!
//! The functions get and put must each run in O(1) average time complexity.
//!
//! Example 1:
//! Input
//! [&"LRUCache&", &"put&", &"put&", &"get&", &"put&", &"get&", &"put&", &"get&", &"get&", &"get&"]
//! [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
//! Output
//! [null, null, null, 1, null, -1, null, -1, 3, 4]
//! Explanation
//! LRUCache lRUCache = new LRUCache(2);
//! lRUCache.put(1, 1); // cache is {1=1}
//! lRUCache.put(2, 2); // cache is {1=1, 2=2}
//! lRUCache.get(1);    // return 1
//! lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
//! lRUCache.get(2);    // returns -1 (not found)
//! lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
//! lRUCache.get(1);    // return -1 (not found)
//! lRUCache.get(3);    // return 3
//! lRUCache.get(4);    // return 4
//!
//! Constraints:
//! - 1 <= capacity <= 3000
//! - 0 <= key <= 10**4
//! - 0 <= value <= 10**5
//! - At most 2 * 10**5 calls will be made to get and put.

use std::{collections::HashMap, hash::Hash, ops::Deref};

#[derive(Debug)]
struct CacheNode<K: Hash, V> {
    key: K,
    value: V,
    previous: Option<usize>,
    next: Option<usize>,
}

impl<K: Hash, V> Deref for CacheNode<K, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<K: Hash, V> CacheNode<K, V> {
    pub fn new(key: K, value: V, previous: Option<usize>, next: Option<usize>) -> Self {
        Self {
            key,
            value,
            previous,
            next,
        }
    }
}

#[derive(Debug)]
struct LRUCacheInner<K: Eq + Hash + Clone, V> {
    newest: Option<usize>,
    oldest: Option<usize>,
    indices: HashMap<K, usize>,
    nodes: Vec<CacheNode<K, V>>,
    capacity: usize,
}

impl<K: Hash + Eq + Clone, V> LRUCacheInner<K, V> {
    /// Initialize a Least Recently Used cache of `capacity` elements.
    pub fn new(capacity: usize) -> Self {
        Self {
            newest: None,
            oldest: None,
            indices: HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Return the value of the key, if it exists.
    /// This bumps the priority of this key to the most recent.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&index) = self.indices.get(key) {
            self.set_to_newest(index);
            Some(&self.nodes[index].value)
        } else {
            None
        }
    }

    /// Update the value `key` to `value`, if it exists.
    /// Otherwise, add the `key-value` pair to the cache.
    /// This bumps the priority of this key to the most recent.
    /// If needed, the least recently used value is evicted from the cache.
    pub fn put(&mut self, key: K, value: V) {
        if let Some(&index) = self.indices.get(&key) {
            self.replace_existing(value, index);
        } else {
            let index = self.indices.len();
            if index == self.capacity {
                self.insert_instead_of_oldest(key, value);
            } else {
                self.insert(key, value, index);
            }
        }
    }

    fn replace_existing(&mut self, value: V, index: usize) {
        self.nodes[index].value = value;
        self.set_to_newest(index);
    }

    fn insert_instead_of_oldest(&mut self, key: K, value: V) {
        let oldest = self.oldest.unwrap();
        self.indices.remove(&self.nodes[oldest].key);
        self.indices.insert(key.clone(), oldest);

        self.nodes[oldest].key = key;
        self.nodes[oldest].value = value;

        self.set_to_newest(oldest);
    }

    fn insert(&mut self, key: K, value: V, index: usize) {
        self.nodes
            .push(CacheNode::new(key.clone(), value, None, None));
        self.indices.insert(key, index);
        self.set_to_newest(index);
    }

    fn set_to_newest(&mut self, index: usize) {
        // 1 <──> 2 <──> 3 <──> 4 <──> 5
        if let Some(newest) = self.newest {
            // Pointing to 5: nothing to be done.
            if index == newest {
                return;
            }

            // Pointing to 3: connect 5 and 3.
            // 1 <──> 2 <──> 3 <──> 4 <──> 5 ┐
            //               ^               │
            //               └───────────────┘
            self.nodes[newest].next = Some(index);
        }

        //        ┌─────────────┐
        //        │             v
        // 1 <──> 2 <─── 3 <──> 4 <──> 5 ┐
        //               ^               │
        //               └───────────────┘
        if let Some(previous) = self.nodes[index].previous {
            self.nodes[previous].next = self.nodes[index].next;
        }
        //        ┌─────────────┐
        //        │             v
        // 1 <──> 2 <─── 3 ───> 4 <──> 5 ┐
        //        ^      ^      │        │
        //        │      └──────┼────────┘
        //        └─────────────┘
        if let Some(next) = self.nodes[index].next {
            self.nodes[next].previous = self.nodes[index].previous;
            // Pointing to oldest node (1): make next (2) the new oldest.
            if self.oldest == Some(index) {
                self.oldest = Some(next);
            }
        }

        //        ┌─────────────┐
        //        │             v
        // 1 <──> 2      3      4 <──> 5
        //        ^      ^      │      ^
        //        │      └──────┼──────┘
        //        └─────────────┘
        // Which is equivalent to:
        // 1 <──> 2 <──> 4 <──> 5 <──> 3
        self.nodes[index].previous = self.newest;
        self.nodes[index].next = None;

        // Bookkeeping the indices.
        self.newest = Some(index);
        if self.oldest.is_none() {
            self.oldest = Some(index);
        }
    }
}

struct LRUCache {
    inner: LRUCacheInner<i32, i32>,
}
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            inner: LRUCacheInner::new(capacity as usize),
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        *self.inner.get(&key).unwrap_or(&-1)
    }
    pub fn put(&mut self, key: i32, value: i32) {
        self.inner.put(key, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_leetcode::medium::s0146_lru_cache::LRUCacheInner;

    #[test]
    fn put_get_without_evict() {
        let mut cache = LRUCacheInner::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        for _ in 0..3 {
            assert_eq!(cache.get(&2), Some(&2));
            assert_eq!(cache.get(&1), Some(&1));
        }
    }

    #[test]
    fn put_replace_does_not_evict() {
        let mut cache = LRUCacheInner::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(2, 3);
        assert_eq!(cache.get(&1), Some(&1));
        assert_eq!(cache.get(&2), Some(&3));
    }

    #[test]
    fn put_new_evicts() {
        let mut cache = LRUCacheInner::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&2));
        assert_eq!(cache.get(&3), Some(&3));
    }

    #[test]
    fn get_absent_does_not_bump_priority() {
        let mut cache = LRUCacheInner::new(2);

        cache.put(1, 1); // 1
        cache.put(2, 2); // 2 > 1
        cache.get(&3); // 2 > 1
        cache.put(3, 3); // 3 > 2
        assert_eq!(cache.get(&1), None);
    }

    #[test]
    fn get_bumps_priority() {
        let mut cache = LRUCacheInner::new(3);

        cache.put(1, 1); // 1
        cache.put(2, 2); // 2 > 1
        cache.put(3, 3); // 3 > 2 > 1
        cache.put(4, 4); // 4 > 3 > 2

        assert_eq!(cache.get(&4), Some(&4)); // 4 > 3 > 2
        assert_eq!(cache.get(&3), Some(&3)); // 3 > 4 > 2
        assert_eq!(cache.get(&2), Some(&2)); // 2 > 3 > 4
        assert_eq!(cache.get(&1), None);

        cache.put(5, 5); // 5 > 2 > 3

        assert_eq!(cache.get(&1), None); // 5 > 2 > 3
        assert_eq!(cache.get(&2), Some(&2)); // 2 > 5 > 3
        assert_eq!(cache.get(&3), Some(&3)); // 3 > 2 > 5
        assert_eq!(cache.get(&4), None); // 3 > 2 > 5
        assert_eq!(cache.get(&5), Some(&5)); // 5 > 3 > 2
    }
}
