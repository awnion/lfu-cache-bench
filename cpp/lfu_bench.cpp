#include "cpp/lfu/btree_lfu.h"
#include "cpp/lfu/heap_lfu.h"
#include "cpp/lfu/linked_lfu.h"
#include "cpp/lfu/types.h"
#include "cpp/lfu/vec_lfu.h"

#include <chrono>
#include <cstddef>
#include <cstdint>
#include <fstream>
#include <functional>
#include <iomanip>
#include <string>
#include <vector>

namespace {

constexpr std::uint64_t kOperations = 250;
constexpr int kSamples = 7;
constexpr std::size_t kCapacities[] = {1, 2, 4, 8, 16, 32, 64, 128, 256, 512,
                                       1024, 2048, 4096, 8192, 16384, 32768};

template <typename Cache> void run_workload(std::size_t capacity, std::uint64_t operations) {
    Cache cache(capacity);
    lfu::Value old_value = 0;

    for (lfu::Key key = 0; key < capacity; ++key) {
        cache.insert(key, key, old_value);
    }

    lfu::Key key_space = capacity * 2;
    for (std::uint64_t i = 0; i < operations; ++i) {
        lfu::Key key = i % key_space;
        if (i % 4 == 0) {
            cache.insert(key, i, old_value);
        } else if (auto value = cache.get(key); value != nullptr) {
            old_value ^= *value;
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
    run_workload<Cache>(capacity, kOperations);

    double total = 0.0;
    for (int sample = 0; sample < kSamples; ++sample) {
        total += measure_us([&] { run_workload<Cache>(capacity, kOperations); });
    }

    results.push_back({capacity, name, total / kSamples});
}

} // namespace

int main() {
    std::vector<Result> results;

    for (std::size_t capacity : kCapacities) {
        bench_one<lfu::BTreeLFU>(results, "btree", capacity);
        bench_one<lfu::VecLFU>(results, "vec", capacity);
        bench_one<lfu::HeapLFU>(results, "heap", capacity);
        bench_one<lfu::LinkedLFU>(results, "linked", capacity);
    }

    std::ofstream file("assets/cpp-lfu-benchmark.csv");
    file << "capacity,implementation,mean_us\n";
    file << std::fixed << std::setprecision(6);
    for (const auto &result : results) {
        file << result.capacity << ',' << result.implementation << ',' << result.mean_us << '\n';
    }
}
