#pragma once

#include "cpp/lfu/types.h"

#include <cstddef>
#include <cstdint>
#include <unordered_map>

namespace lfu {

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

} // namespace lfu
