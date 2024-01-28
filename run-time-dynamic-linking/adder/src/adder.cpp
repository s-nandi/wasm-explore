#include "adder.hpp"

namespace adder {

Worker::Worker(int value) : value{value} {}

auto Worker::apply(int delta) -> void {
    value += delta;
}

auto Worker::get() -> int {
    return value;
}

}  // namespace adder

extern "C" auto example(int value) -> int {
    auto adder = adder::Worker{value};
    adder.apply(2);
    return adder.get();
}