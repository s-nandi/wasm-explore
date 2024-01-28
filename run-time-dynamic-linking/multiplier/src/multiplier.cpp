#include "multiplier.hpp"

namespace multiplier {

Worker::Worker(int value) : value{value} {}

auto Worker::apply(int coefficient) -> void {
    value *= coefficient;
}

auto Worker::get() -> int {
    return value;
}

}  // namespace multiplier

extern "C" auto example(int value) -> int {
    auto multiplier = multiplier::Worker{value};
    multiplier.apply(2);
    return multiplier.get();
}
