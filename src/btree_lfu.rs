use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

/// LFU cache based on HashMap storage and BTreeSet frequency index.
pub struct LFU<K, V> {
    // NOTE:
    // We want to: 1) get frequency and value by key, 2) get key by frequency,
    // 3) and keep insert/delete reasonably simple and synchronized, which lets us change frequency.
    // The trick is that instead of a pointer like NonNull<...> and unsafe { ...get_mut() },
    // we store tuples: (freq, key) and (freq, value).
    // TRADEOFF: We include the key in the index to make entries unique,
    // so we have to require Ord.
    values: HashMap<K, (u32, V)>,
    index: BTreeSet<(u32, K)>,
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
            index: BTreeSet::new(),
            max_len,
        }
    }

    /// If `key` is in LFU then this will return associated value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let entry = self.values.get_mut(key)?;

        // NOTE: This is not very optimal, but it avoids unsafe.
        let (_, key) = self.index.take(&(entry.0, *key))?;
        entry.0 += 1;
        self.index.insert((entry.0, key));

        Some(&entry.1)
    }

    /// Insert a new `value` with given `key`. If LFU already has a value
    /// associated with given key, the value is replaced (and old value is returned).
    ///
    /// If LFU cache already has `max_len` elements, then least frequently used (get/put) key is
    /// removed from cache to keep it at `max_len` after insertion.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.values.remove(&key) {
            Some((frequency, old_value)) => {
                self.index.remove(&(frequency, key));

                self.index.insert((1, key));
                self.values.insert(key, (1, value));

                Some(old_value)
            }
            None if self.values.len() == self.max_len => {
                let (_, k) = self.index.pop_first().unwrap();
                let (_, old_value) = self.values.remove(&k).unwrap();

                self.index.insert((1, key));
                self.values.insert(key, (1, value));

                Some(old_value)
            }
            None => {
                self.index.insert((1, key));
                self.values.insert(key, (1, value));

                None
            }
        }
    }
}
