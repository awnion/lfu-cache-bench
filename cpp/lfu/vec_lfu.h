#pragma once

#include "cpp/lfu/types.h"

#include <cstddef>
#include <cstdint>
#include <unordered_map>
#include <utility>
#include <vector>

namespace lfu {

class VecLFU {
  public:
    explicit VecLFU(std::size_t max_len) : max_len_(max_len) {}

    const Value *get(const Key &key) {
        auto index = positions_.find(key);
        if (index == positions_.end()) {
            return nullptr;
        }

        auto &entry = entries_[index->second];
        entry.frequency += 1;
        sort_from(index->second);
        return &values_.find(key)->second;
    }

    bool insert(Key key, Value value, Value &old_value) {
        auto existing = values_.find(key);
        if (existing != values_.end()) {
            old_value = existing->second;
            existing->second = value;
            entries_[positions_[key]].frequency = 1;
            sort_to_front(positions_[key]);
            return true;
        }

        if (values_.size() == max_len_) {
            Key victim_key = entries_.front().key;
            old_value = values_[victim_key];
            values_.erase(victim_key);
            positions_.erase(victim_key);
            entries_.erase(entries_.begin());
            rebuild_positions();
            values_.emplace(key, value);
            entries_.push_back({1, key});
            positions_[key] = entries_.size() - 1;
            sort_to_front(entries_.size() - 1);
            return true;
        }

        values_.emplace(key, value);
        entries_.push_back({1, key});
        positions_[key] = entries_.size() - 1;
        sort_to_front(entries_.size() - 1);
        return false;
    }

  private:
    struct Entry {
        std::uint32_t frequency;
        Key key;
    };

    static bool less(const Entry &left, const Entry &right) {
        return std::pair{left.frequency, left.key} < std::pair{right.frequency, right.key};
    }

    void swap_entries(std::size_t left, std::size_t right) {
        std::swap(entries_[left], entries_[right]);
        positions_[entries_[left].key] = left;
        positions_[entries_[right].key] = right;
    }

    void sort_from(std::size_t index) {
        while (index + 1 < entries_.size() && less(entries_[index + 1], entries_[index])) {
            swap_entries(index, index + 1);
            index += 1;
        }
    }

    void sort_to_front(std::size_t index) {
        while (index > 0 && less(entries_[index], entries_[index - 1])) {
            swap_entries(index, index - 1);
            index -= 1;
        }
    }

    void rebuild_positions() {
        for (std::size_t i = 0; i < entries_.size(); ++i) {
            positions_[entries_[i].key] = i;
        }
    }

    std::unordered_map<Key, Value> values_;
    std::unordered_map<Key, std::size_t> positions_;
    std::vector<Entry> entries_;
    std::size_t max_len_;
};

} // namespace lfu
