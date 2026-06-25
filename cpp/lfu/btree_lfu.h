#pragma once

#include "cpp/lfu/types.h"

#include <cstddef>
#include <cstdint>
#include <set>
#include <unordered_map>

namespace lfu {

class BTreeLFU {
  public:
    explicit BTreeLFU(std::size_t max_len) : max_len_(max_len) {}

    const Value *get(const Key &key) {
        auto entry = values_.find(key);
        if (entry == values_.end()) {
            return nullptr;
        }

        index_.erase({entry->second.frequency, key});
        entry->second.frequency += 1;
        index_.insert({entry->second.frequency, key});
        return &entry->second.value;
    }

    bool insert(Key key, Value value, Value &old_value) {
        auto entry = values_.find(key);
        if (entry != values_.end()) {
            old_value = entry->second.value;
            index_.erase({entry->second.frequency, key});
            entry->second = {1, value};
            index_.insert({1, key});
            return true;
        }

        if (values_.size() == max_len_) {
            auto victim = index_.begin();
            Key victim_key = victim->second;
            index_.erase(victim);
            old_value = values_.find(victim_key)->second.value;
            values_.erase(victim_key);
            values_.emplace(key, Entry{1, value});
            index_.insert({1, key});
            return true;
        }

        values_.emplace(key, Entry{1, value});
        index_.insert({1, key});
        return false;
    }

  private:
    struct Entry {
        std::uint32_t frequency;
        Value value;
    };

    std::unordered_map<Key, Entry> values_;
    std::set<std::pair<std::uint32_t, Key>> index_;
    std::size_t max_len_;
};

} // namespace lfu
