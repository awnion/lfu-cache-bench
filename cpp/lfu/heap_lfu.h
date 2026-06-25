#pragma once

#include "cpp/lfu/types.h"

#include <cstddef>
#include <cstdint>
#include <unordered_map>
#include <utility>
#include <vector>

namespace lfu {

class HeapLFU {
  public:
    explicit HeapLFU(std::size_t max_len) : max_len_(max_len) {}

    const Value *get(const Key &key) {
        auto entry = values_.find(key);
        if (entry == values_.end()) {
            return nullptr;
        }

        entry->second.frequency += 1;
        heap_[entry->second.heap_index].frequency += 1;
        sift_down(entry->second.heap_index);
        return &entry->second.value;
    }

    bool insert(Key key, Value value, Value &old_value) {
        auto entry = values_.find(key);
        if (entry != values_.end()) {
            old_value = entry->second.value;
            entry->second.value = value;
            entry->second.frequency = 1;
            heap_[entry->second.heap_index] = {1, key};
            sift_up(entry->second.heap_index);
            return true;
        }

        if (values_.size() == max_len_) {
            auto victim = pop_heap();
            old_value = values_[victim.key].value;
            values_.erase(victim.key);
            push_heap(1, key, value);
            return true;
        }

        push_heap(1, key, value);
        return false;
    }

  private:
    struct HeapEntry {
        std::uint32_t frequency;
        Key key;
    };

    struct ValueEntry {
        std::uint32_t frequency;
        Value value;
        std::size_t heap_index;
    };

    static bool less(const HeapEntry &left, const HeapEntry &right) {
        return std::pair{left.frequency, left.key} < std::pair{right.frequency, right.key};
    }

    void push_heap(std::uint32_t frequency, Key key, Value value) {
        std::size_t index = heap_.size();
        heap_.push_back({frequency, key});
        values_[key] = {frequency, value, index};
        sift_up(index);
    }

    HeapEntry pop_heap() {
        swap_heap(0, heap_.size() - 1);
        HeapEntry removed = heap_.back();
        heap_.pop_back();
        if (!heap_.empty()) {
            sift_down(0);
        }
        return removed;
    }

    void sift_up(std::size_t index) {
        while (index > 0) {
            std::size_t parent = (index - 1) / 2;
            if (!less(heap_[index], heap_[parent])) {
                break;
            }
            swap_heap(index, parent);
            index = parent;
        }
    }

    void sift_down(std::size_t index) {
        while (true) {
            std::size_t left = index * 2 + 1;
            std::size_t right = left + 1;
            std::size_t smallest = index;

            if (left < heap_.size() && less(heap_[left], heap_[smallest])) {
                smallest = left;
            }
            if (right < heap_.size() && less(heap_[right], heap_[smallest])) {
                smallest = right;
            }
            if (smallest == index) {
                break;
            }
            swap_heap(index, smallest);
            index = smallest;
        }
    }

    void swap_heap(std::size_t left, std::size_t right) {
        std::swap(heap_[left], heap_[right]);
        values_[heap_[left].key].heap_index = left;
        values_[heap_[right].key].heap_index = right;
    }

    std::unordered_map<Key, ValueEntry> values_;
    std::vector<HeapEntry> heap_;
    std::size_t max_len_;
};

} // namespace lfu
