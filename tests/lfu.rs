mod btree_lfu_tests {
    use workato::btree_lfu::LFU;

    #[test]
    #[should_panic(expected = "Should be bigger than 0")]
    fn new_panics_on_zero_capacity() {
        let _: LFU<u64, &str> = LFU::new(0);
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");

        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"a"));
    }

    #[test]
    fn get_increases_frequency_and_protects_key_from_eviction() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        assert_eq!(cache.get(&1), Some(&"a"));

        assert_eq!(cache.insert(3, "c"), Some("b"));
        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn capacity_one_keeps_only_latest_inserted_key() {
        let mut cache = LFU::new(1);

        assert_eq!(cache.insert(1, "a"), None);
        assert_eq!(cache.insert(2, "b"), Some("a"));

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
    }

    #[test]
    fn evicts_smallest_key_when_frequencies_tie() {
        let mut cache = LFU::new(2);

        cache.insert(2, "b");
        cache.insert(1, "a");

        assert_eq!(cache.insert(3, "c"), Some("a"));
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn repeated_hits_survive_multiple_evictions() {
        let mut cache = LFU::new(3);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        cache.get(&1);
        cache.get(&1);
        cache.get(&2);

        assert_eq!(cache.insert(4, "d"), Some("c"));
        assert_eq!(cache.insert(5, "e"), Some("d"));

        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), None);
        assert_eq!(cache.get(&4), None);
        assert_eq!(cache.get(&5), Some(&"e"));
    }

    #[test]
    fn insert_existing_key_before_capacity_returns_old_value() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");

        assert_eq!(cache.insert(1, "new-a"), Some("a"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
    }

    #[test]
    fn insert_existing_key_at_capacity_replaces_without_eviction() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.get(&1);

        assert_eq!(cache.insert(1, "new-a"), Some("a"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
        assert_eq!(cache.get(&2), Some(&"b"));
    }
}

mod vec_lfu_tests {
    use workato::vec_lfu::LFU;

    #[test]
    #[should_panic(expected = "Should be bigger than 0")]
    fn new_panics_on_zero_capacity() {
        let _: LFU<u64, &str> = LFU::new(0);
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");

        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"a"));
    }

    #[test]
    fn get_increases_frequency_and_protects_key_from_eviction() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        assert_eq!(cache.get(&1), Some(&"a"));

        assert_eq!(cache.insert(3, "c"), Some("b"));
        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn capacity_one_keeps_only_latest_inserted_key() {
        let mut cache = LFU::new(1);

        assert_eq!(cache.insert(1, "a"), None);
        assert_eq!(cache.insert(2, "b"), Some("a"));

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
    }

    #[test]
    fn evicts_smallest_key_when_frequencies_tie() {
        let mut cache = LFU::new(2);

        cache.insert(2, "b");
        cache.insert(1, "a");

        assert_eq!(cache.insert(3, "c"), Some("a"));
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn repeated_hits_survive_multiple_evictions() {
        let mut cache = LFU::new(3);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        cache.get(&1);
        cache.get(&1);
        cache.get(&2);

        assert_eq!(cache.insert(4, "d"), Some("c"));
        assert_eq!(cache.insert(5, "e"), Some("d"));

        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), None);
        assert_eq!(cache.get(&4), None);
        assert_eq!(cache.get(&5), Some(&"e"));
    }

    #[test]
    fn insert_existing_key_before_capacity_returns_old_value() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");

        assert_eq!(cache.insert(1, "new-a"), Some("a"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
    }

    #[test]
    fn insert_existing_key_at_capacity_replaces_without_eviction() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.get(&1);

        assert_eq!(cache.insert(1, "new-a"), Some("a"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
        assert_eq!(cache.get(&2), Some(&"b"));
    }
}

mod heap_lfu_tests {
    use workato::heap_lfu::LFU;

    #[test]
    #[should_panic(expected = "Should be bigger than 0")]
    fn new_panics_on_zero_capacity() {
        let _: LFU<u64, &str> = LFU::new(0);
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");

        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"a"));
    }

    #[test]
    fn get_increases_frequency_and_protects_key_from_eviction() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        assert_eq!(cache.get(&1), Some(&"a"));

        assert_eq!(cache.insert(3, "c"), Some("b"));
        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn capacity_one_keeps_only_latest_inserted_key() {
        let mut cache = LFU::new(1);

        assert_eq!(cache.insert(1, "a"), None);
        assert_eq!(cache.insert(2, "b"), Some("a"));

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
    }

    #[test]
    fn evicts_smallest_key_when_frequencies_tie() {
        let mut cache = LFU::new(2);

        cache.insert(2, "b");
        cache.insert(1, "a");

        assert_eq!(cache.insert(3, "c"), Some("a"));
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn repeated_hits_survive_multiple_evictions() {
        let mut cache = LFU::new(3);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(3, "c");

        cache.get(&1);
        cache.get(&1);
        cache.get(&2);

        assert_eq!(cache.insert(4, "d"), Some("c"));
        assert_eq!(cache.insert(5, "e"), Some("d"));

        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), None);
        assert_eq!(cache.get(&4), None);
        assert_eq!(cache.get(&5), Some(&"e"));
    }

    #[test]
    fn insert_existing_key_before_capacity_returns_old_value() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");

        assert_eq!(cache.insert(1, "new-a"), Some("a"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
    }

    #[test]
    fn insert_existing_key_at_capacity_replaces_without_eviction() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.get(&1);

        assert_eq!(cache.insert(1, "new-a"), Some("a"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
        assert_eq!(cache.get(&2), Some(&"b"));
    }

    #[test]
    fn heap_index_update_does_not_evict_current_hot_key() {
        let mut cache = LFU::new(2);

        cache.insert(1, "a");
        cache.insert(2, "b");
        cache.insert(1, "new-a");
        cache.get(&1);

        assert_eq!(cache.insert(3, "c"), Some("b"));
        assert_eq!(cache.get(&1), Some(&"new-a"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"c"));
    }
}
