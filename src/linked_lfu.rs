use std::{collections::HashMap, hash::Hash, ptr::NonNull};

/// LFU cache based on HashMap pointers and frequency-bucketed doubly linked lists.
pub struct LFU<K, V> {
    nodes: HashMap<K, NonNull<Node<K, V>>>,
    frequencies: HashMap<u32, List<K, V>>,
    min_frequency: u32,
    max_len: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
    frequency: u32,
    prev: Option<NonNull<Node<K, V>>>,
    next: Option<NonNull<Node<K, V>>>,
}

struct List<K, V> {
    head: Option<NonNull<Node<K, V>>>,
    tail: Option<NonNull<Node<K, V>>>,
    len: usize,
}

impl<K, V> Default for List<K, V> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<K: Hash + Ord + Copy, V> LFU<K, V> {
    /// `max_len` - maximum number of items that this cache can hold.
    pub fn new(max_len: usize) -> Self {
        if max_len < 1 {
            panic!("Should be bigger than 0");
        }
        Self {
            nodes: HashMap::with_capacity(max_len),
            frequencies: HashMap::new(),
            min_frequency: 0,
            max_len,
        }
    }

    /// If `key` is in LFU then this will return associated value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let node = *self.nodes.get(key)?;
        self.increment_frequency(node);

        Some(unsafe { &(*node.as_ptr()).value })
    }

    /// Insert a new `value` with given `key`. If LFU already has a value
    /// associated with given key, the value is replaced (and old value is returned).
    ///
    /// If LFU cache already has `max_len` elements, then least frequently used (get/put) key is
    /// removed from cache to keep it at `max_len` after insertion.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(&node) = self.nodes.get(&key) {
            let old_value = unsafe {
                let node = &mut *node.as_ptr();
                std::mem::replace(&mut node.value, value)
            };
            self.move_to_frequency(node, 1);
            self.min_frequency = 1;
            return Some(old_value);
        }

        if self.nodes.len() == self.max_len {
            let evicted = self
                .frequencies
                .get_mut(&self.min_frequency)
                .unwrap()
                .pop_front()
                .unwrap();
            let old = self.drop_node(evicted);
            self.clean_frequency(self.min_frequency);
            self.insert_node(key, value);
            return Some(old.value);
        }

        self.insert_node(key, value);
        None
    }

    fn insert_node(&mut self, key: K, value: V) {
        let node = Box::new(Node {
            key,
            value,
            frequency: 1,
            prev: None,
            next: None,
        });
        let node = NonNull::from(Box::leak(node));

        self.nodes.insert(key, node);
        self.frequencies.entry(1).or_default().insert_sorted(node);
        self.min_frequency = 1;
    }

    fn increment_frequency(&mut self, node: NonNull<Node<K, V>>) {
        let old_frequency = unsafe { (*node.as_ptr()).frequency };
        self.move_to_frequency(node, old_frequency + 1);

        if self.min_frequency == old_frequency && !self.frequencies.contains_key(&old_frequency) {
            self.min_frequency += 1;
        }
    }

    fn move_to_frequency(&mut self, node: NonNull<Node<K, V>>, new_frequency: u32) {
        let old_frequency = unsafe { (*node.as_ptr()).frequency };
        self.frequencies.get_mut(&old_frequency).unwrap().unlink(node);
        self.clean_frequency(old_frequency);

        unsafe {
            (*node.as_ptr()).frequency = new_frequency;
        }
        self.frequencies
            .entry(new_frequency)
            .or_default()
            .insert_sorted(node);
    }

    fn clean_frequency(&mut self, frequency: u32) {
        if self
            .frequencies
            .get(&frequency)
            .is_some_and(|list| list.is_empty())
        {
            self.frequencies.remove(&frequency);
        }
    }

    fn drop_node(&mut self, node: NonNull<Node<K, V>>) -> Box<Node<K, V>> {
        let key = unsafe { (*node.as_ptr()).key };
        self.nodes.remove(&key);

        unsafe { Box::from_raw(node.as_ptr()) }
    }
}

impl<K, V> Drop for LFU<K, V> {
    fn drop(&mut self) {
        for &node in self.nodes.values() {
            unsafe {
                drop(Box::from_raw(node.as_ptr()));
            }
        }
    }
}

impl<K: Ord + Copy, V> List<K, V> {
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn insert_sorted(&mut self, node: NonNull<Node<K, V>>) {
        let mut current = self.head;
        while let Some(current_node) = current {
            if Self::key(node) < Self::key(current_node) {
                self.insert_before(node, current_node);
                return;
            }
            current = unsafe { (*current_node.as_ptr()).next };
        }

        self.push_back(node);
    }

    fn pop_front(&mut self) -> Option<NonNull<Node<K, V>>> {
        let node = self.head?;
        self.unlink(node);
        Some(node)
    }

    fn unlink(&mut self, node: NonNull<Node<K, V>>) {
        let (prev, next) = unsafe { ((*node.as_ptr()).prev, (*node.as_ptr()).next) };

        if let Some(prev) = prev {
            unsafe {
                (*prev.as_ptr()).next = next;
            }
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            unsafe {
                (*next.as_ptr()).prev = prev;
            }
        } else {
            self.tail = prev;
        }

        unsafe {
            (*node.as_ptr()).prev = None;
            (*node.as_ptr()).next = None;
        }
        self.len -= 1;
    }

    fn insert_before(&mut self, node: NonNull<Node<K, V>>, next: NonNull<Node<K, V>>) {
        let prev = unsafe { (*next.as_ptr()).prev };

        unsafe {
            (*node.as_ptr()).prev = prev;
            (*node.as_ptr()).next = Some(next);
            (*next.as_ptr()).prev = Some(node);
        }

        if let Some(prev) = prev {
            unsafe {
                (*prev.as_ptr()).next = Some(node);
            }
        } else {
            self.head = Some(node);
        }
        self.len += 1;
    }

    fn push_back(&mut self, node: NonNull<Node<K, V>>) {
        unsafe {
            (*node.as_ptr()).prev = self.tail;
            (*node.as_ptr()).next = None;
        }

        if let Some(tail) = self.tail {
            unsafe {
                (*tail.as_ptr()).next = Some(node);
            }
        } else {
            self.head = Some(node);
        }

        self.tail = Some(node);
        self.len += 1;
    }

    fn key(node: NonNull<Node<K, V>>) -> K {
        unsafe { (*node.as_ptr()).key }
    }
}
