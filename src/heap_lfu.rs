use std::{collections::HashMap, hash::Hash};

/// LFU cache based on HashMap storage and a tuple min-heap.
pub struct LFU<K, V> {
    values: HashMap<K, (u32, V, usize)>,
    // NOTE: std::BinaryHeap does not expose stable element indexes.
    // This custom heap keeps `values` indexes in sync during every sift/swap.
    heap: Vec<(u32, K)>,
    max_len: usize,
}

impl<K: Hash + Ord + Copy, V> LFU<K, V> {
    /// `max_len` - maximum number of items that this cache can hold.
    pub fn new(max_len: usize) -> Self {
        if max_len < 1 {
            panic!("Should be bigger than 0");
        }
        Self {
            values: HashMap::with_capacity(max_len),
            heap: Vec::new(),
            max_len,
        }
    }

    /// If `key` is in LFU then this will return associated value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let (index, value) = {
            let entry = self.values.get_mut(key)?;
            entry.0 += 1;
            (entry.2, &entry.1 as *const V)
        };

        self.heap[index].0 += 1;
        self.sift_down(index);

        // `sift_down` only swaps heap tuples and updates indices in existing map entries.
        // It does not insert/remove from `values`, so the value pointer remains valid.
        Some(unsafe { &*value })
    }

    /// Insert a new `value` with given `key`. If LFU already has a value
    /// associated with given key, the value is replaced (and old value is returned).
    ///
    /// If LFU cache already has `max_len` elements, then least frequently used (get/put) key is
    /// removed from cache to keep it at `max_len` after insertion.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(entry) = self.values.get_mut(&key) {
            let old_value = std::mem::replace(&mut entry.1, value);
            let index = entry.2;
            entry.0 = 1;

            self.heap[index] = (1, key);
            self.sift_up(index);

            Some(old_value)
        } else if self.values.len() == self.max_len {
            let (_, evicted_key) = self.pop_heap().unwrap();
            let (_, old_value, _) = self.values.remove(&evicted_key).unwrap();

            self.push_heap(1, key, value);

            Some(old_value)
        } else {
            self.push_heap(1, key, value);
            None
        }
    }

    fn push_heap(&mut self, frequency: u32, key: K, value: V) {
        let index = self.heap.len();
        self.heap.push((frequency, key));
        self.values.insert(key, (frequency, value, index));
        self.sift_up(index);
    }

    fn sift_up(&mut self, index: usize) -> usize {
        let mut index = index;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.heap[parent] <= self.heap[index] {
                break;
            }

            self.swap_heap(parent, index);
            index = parent;
        }

        index
    }

    fn pop_heap(&mut self) -> Option<(u32, K)> {
        if self.heap.is_empty() {
            return None;
        }

        let last_index = self.heap.len() - 1;
        self.swap_heap(0, last_index);
        let removed = self.heap.pop();
        if !self.heap.is_empty() {
            self.sift_down(0);
        }

        removed
    }

    fn sift_down(&mut self, index: usize) -> usize {
        let mut index = index;
        loop {
            let left = index * 2 + 1;
            let right = left + 1;
            let mut smallest = index;

            if left < self.heap.len() && self.heap[left] < self.heap[smallest] {
                smallest = left;
            }
            if right < self.heap.len() && self.heap[right] < self.heap[smallest] {
                smallest = right;
            }
            if smallest == index {
                break;
            }

            self.swap_heap(index, smallest);
            index = smallest;
        }

        index
    }

    fn swap_heap(&mut self, a: usize, b: usize) {
        self.heap.swap(a, b);

        let left_key = self.heap[a].1;
        let right_key = self.heap[b].1;

        self.values.get_mut(&left_key).unwrap().2 = a;
        self.values.get_mut(&right_key).unwrap().2 = b;
    }
}
