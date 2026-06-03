use std::{collections::HashMap, hash::Hash};

/// LFU cache based on HashMap storage and Vec
pub struct LFU<K, V> {
    table: HashMap<K, usize>,
    values: Vec<(K, u32, V)>,
    max_len: usize,
}

impl<K: Hash + Ord + Copy, V> LFU<K, V> {
    /// `max_len` - maximum number of items that this cache can hold.
    pub fn new(max_len: usize) -> Self {
        if max_len < 1 {
            panic!("Should be bigger than 0");
        }
        Self {
            table: HashMap::with_capacity(max_len),
            values: vec![],
            max_len,
        }
    }

    /// If `key` is in LFU then this will return associated value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let mut index = *self.table.get(key)?;

        let new_f = self.values[index].1 + 1;
        while index > 0 && self.values[index - 1].1 < new_f {
            self.values.swap(index, index - 1);
            *self.table.get_mut(&self.values[index].0).unwrap() += 1;
            index -= 1;
            *self.table.get_mut(&self.values[index].0).unwrap() -= 1;
        }

        self.values[index].1 += 1;
        Some(&self.values[index].2)
    }

    /// Insert a new `value` with given `key`. If LFU already has a value
    /// associated with given key, the value is replaced (and old value is returned).
    ///
    /// If LFU cache already has `max_len` elements, then least frequently used (get/put) key is
    /// removed from cache to keep it at `max_len` after insertion.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.table.remove(&key) {
            Some(mut i) => {
                while i < self.values.len() - 1 {
                    self.values.swap(i, i + 1);
                    *self.table.get_mut(&self.values[i].0).unwrap() -= 1;
                    i += 1;
                }
                let old_value = self.values.pop().unwrap();
                self.values.push((key, 1, value));
                self.table.insert(key, self.values.len() - 1);
                Some(old_value.2)
            }
            None if self.values.len() == self.max_len => {
                let old_value = self.values.pop().unwrap();
                self.table.remove(&old_value.0);

                self.values.push((key, 1, value));
                self.table.insert(key, self.values.len() - 1);

                Some(old_value.2)
            }
            None => {
                self.values.push((key, 1, value));
                self.table.insert(key, self.values.len() - 1);

                None
            }
        }
    }
}
