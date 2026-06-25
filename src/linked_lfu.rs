use std::{collections::HashMap, hash::Hash};

/// LFU cache based on HashMap storage and an indexed doubly linked list.
pub struct LFU<K, V> {
    nodes: Vec<Node<K, V>>,
    index: HashMap<K, usize>,
    free: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
    max_len: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
    frequency: u32,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<K: Hash + Ord + Copy, V> LFU<K, V> {
    /// `max_len` - maximum number of items that this cache can hold.
    pub fn new(max_len: usize) -> Self {
        if max_len < 1 {
            panic!("Should be bigger than 0");
        }
        Self {
            nodes: Vec::with_capacity(max_len),
            index: HashMap::with_capacity(max_len),
            free: Vec::new(),
            head: None,
            tail: None,
            max_len,
        }
    }

    /// If `key` is in LFU then this will return associated value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = *self.index.get(key)?;
        self.nodes[index].frequency += 1;
        self.reposition(index);

        Some(&self.nodes[index].value)
    }

    /// Insert a new `value` with given `key`. If LFU already has a value
    /// associated with given key, the value is replaced (and old value is returned).
    ///
    /// If LFU cache already has `max_len` elements, then least frequently used (get/put) key is
    /// removed from cache to keep it at `max_len` after insertion.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(&index) = self.index.get(&key) {
            let old_value = std::mem::replace(&mut self.nodes[index].value, value);
            self.nodes[index].frequency = 1;
            self.reposition(index);

            return Some(old_value);
        }

        let old_value = if self.index.len() == self.max_len {
            let evicted = self.head.unwrap();
            let old_key = self.nodes[evicted].key;
            self.unlink(evicted);
            self.index.remove(&old_key);

            let old_node = std::mem::replace(
                &mut self.nodes[evicted],
                Node {
                    key,
                    value,
                    frequency: 1,
                    prev: None,
                    next: None,
                },
            );
            self.index.insert(key, evicted);
            self.insert_sorted(evicted);

            return Some(old_node.value);
        } else {
            None
        };

        let index = if let Some(index) = self.free.pop() {
            self.nodes[index] = Node {
                key,
                value,
                frequency: 1,
                prev: None,
                next: None,
            };
            index
        } else {
            self.nodes.push(Node {
                key,
                value,
                frequency: 1,
                prev: None,
                next: None,
            });
            self.nodes.len() - 1
        };

        self.index.insert(key, index);
        self.insert_sorted(index);

        old_value
    }

    fn reposition(&mut self, index: usize) {
        self.unlink(index);
        self.insert_sorted(index);
    }

    fn insert_sorted(&mut self, index: usize) {
        let mut current = self.head;
        while let Some(current_index) = current {
            if self.node_key(index) < self.node_key(current_index) {
                self.insert_before(index, current_index);
                return;
            }
            current = self.nodes[current_index].next;
        }

        self.push_back(index);
    }

    fn node_key(&self, index: usize) -> (u32, K) {
        (self.nodes[index].frequency, self.nodes[index].key)
    }

    fn insert_before(&mut self, index: usize, next: usize) {
        let prev = self.nodes[next].prev;
        self.nodes[index].prev = prev;
        self.nodes[index].next = Some(next);
        self.nodes[next].prev = Some(index);

        if let Some(prev) = prev {
            self.nodes[prev].next = Some(index);
        } else {
            self.head = Some(index);
        }
    }

    fn push_back(&mut self, index: usize) {
        self.nodes[index].prev = self.tail;
        self.nodes[index].next = None;

        if let Some(tail) = self.tail {
            self.nodes[tail].next = Some(index);
        } else {
            self.head = Some(index);
        }

        self.tail = Some(index);
    }

    fn unlink(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;

        if let Some(prev) = prev {
            self.nodes[prev].next = next;
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            self.nodes[next].prev = prev;
        } else {
            self.tail = prev;
        }

        self.nodes[index].prev = None;
        self.nodes[index].next = None;
    }
}
