#include <algorithm>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

using Key = std::uint64_t;
using Value = std::uint64_t;

constexpr std::uint64_t OPERATIONS = 250;
constexpr int SAMPLES = 7;
constexpr std::size_t CAPACITIES[] = {1, 2, 4, 8, 16, 32, 64, 128, 256, 512,
                                      1024, 2048, 4096, 8192, 16384, 32768};

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

class LinkedLFU {
  public:
    explicit LinkedLFU(std::size_t max_len) : max_len_(max_len) {}

    ~LinkedLFU() {
        for (auto &[_, node] : nodes_) {
            delete node;
        }
    }

    const Value *get(const Key &key) {
        auto entry = nodes_.find(key);
        if (entry == nodes_.end()) {
            return nullptr;
        }
        increment_frequency(entry->second);
        return &entry->second->value;
    }

    bool insert(Key key, Value value, Value &old_value) {
        auto entry = nodes_.find(key);
        if (entry != nodes_.end()) {
            Node *node = entry->second;
            old_value = node->value;
            node->value = value;
            move_to_frequency(node, 1);
            min_frequency_ = 1;
            return true;
        }

        if (nodes_.size() == max_len_) {
            Node *victim = frequencies_[min_frequency_].pop_front();
            old_value = victim->value;
            nodes_.erase(victim->key);
            clean_frequency(min_frequency_);
            delete victim;
            insert_node(key, value);
            return true;
        }

        insert_node(key, value);
        return false;
    }

  private:
    struct Node {
        Key key;
        Value value;
        std::uint32_t frequency;
        Node *prev = nullptr;
        Node *next = nullptr;
    };

    struct List {
        Node *head = nullptr;
        Node *tail = nullptr;
        std::size_t len = 0;

        bool empty() const { return len == 0; }

        void insert_sorted(Node *node) {
            for (Node *current = head; current != nullptr; current = current->next) {
                if (node->key < current->key) {
                    insert_before(node, current);
                    return;
                }
            }
            push_back(node);
        }

        Node *pop_front() {
            Node *node = head;
            unlink(node);
            return node;
        }

        void unlink(Node *node) {
            if (node->prev != nullptr) {
                node->prev->next = node->next;
            } else {
                head = node->next;
            }

            if (node->next != nullptr) {
                node->next->prev = node->prev;
            } else {
                tail = node->prev;
            }

            node->prev = nullptr;
            node->next = nullptr;
            len -= 1;
        }

        void insert_before(Node *node, Node *next) {
            node->prev = next->prev;
            node->next = next;
            next->prev = node;

            if (node->prev != nullptr) {
                node->prev->next = node;
            } else {
                head = node;
            }
            len += 1;
        }

        void push_back(Node *node) {
            node->prev = tail;
            node->next = nullptr;
            if (tail != nullptr) {
                tail->next = node;
            } else {
                head = node;
            }
            tail = node;
            len += 1;
        }
    };

    void insert_node(Key key, Value value) {
        Node *node = new Node{key, value, 1};
        nodes_[key] = node;
        frequencies_[1].insert_sorted(node);
        min_frequency_ = 1;
    }

    void increment_frequency(Node *node) {
        std::uint32_t old_frequency = node->frequency;
        move_to_frequency(node, old_frequency + 1);
        if (min_frequency_ == old_frequency && !frequencies_.contains(old_frequency)) {
            min_frequency_ += 1;
        }
    }

    void move_to_frequency(Node *node, std::uint32_t new_frequency) {
        std::uint32_t old_frequency = node->frequency;
        frequencies_[old_frequency].unlink(node);
        clean_frequency(old_frequency);
        node->frequency = new_frequency;
        frequencies_[new_frequency].insert_sorted(node);
    }

    void clean_frequency(std::uint32_t frequency) {
        auto entry = frequencies_.find(frequency);
        if (entry != frequencies_.end() && entry->second.empty()) {
            frequencies_.erase(entry);
        }
    }

    std::unordered_map<Key, Node *> nodes_;
    std::unordered_map<std::uint32_t, List> frequencies_;
    std::uint32_t min_frequency_ = 0;
    std::size_t max_len_;
};

template <typename Cache> void run_workload(std::size_t capacity, std::uint64_t operations) {
    Cache cache(capacity);
    Value old_value = 0;

    for (Key key = 0; key < capacity; ++key) {
        cache.insert(key, key, old_value);
    }

    Key key_space = capacity * 2;
    for (std::uint64_t i = 0; i < operations; ++i) {
        Key key = i % key_space;
        if (i % 4 == 0) {
            cache.insert(key, i, old_value);
        } else {
            auto value = cache.get(key);
            if (value != nullptr) {
                old_value ^= *value;
            }
        }
    }
}

double measure_us(const std::function<void()> &workload) {
    auto start = std::chrono::steady_clock::now();
    workload();
    auto end = std::chrono::steady_clock::now();
    return std::chrono::duration<double, std::micro>(end - start).count();
}

struct Result {
    std::size_t capacity;
    std::string implementation;
    double mean_us;
};

template <typename Cache>
void bench_one(std::vector<Result> &results, const std::string &name, std::size_t capacity) {
    run_workload<Cache>(capacity, OPERATIONS);

    double total = 0.0;
    for (int sample = 0; sample < SAMPLES; ++sample) {
        total += measure_us([&] { run_workload<Cache>(capacity, OPERATIONS); });
    }

    results.push_back({capacity, name, total / SAMPLES});
}

int main() {
    std::vector<Result> results;

    for (std::size_t capacity : CAPACITIES) {
        bench_one<BTreeLFU>(results, "btree", capacity);
        bench_one<VecLFU>(results, "vec", capacity);
        bench_one<HeapLFU>(results, "heap", capacity);
        bench_one<LinkedLFU>(results, "linked", capacity);
    }

    std::ofstream file("assets/cpp-lfu-benchmark.csv");
    file << "capacity,implementation,mean_us\n";
    file << std::fixed << std::setprecision(6);
    for (const auto &result : results) {
        file << result.capacity << ',' << result.implementation << ',' << result.mean_us << '\n';
    }
}
