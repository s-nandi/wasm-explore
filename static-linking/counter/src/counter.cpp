#include "counter.hpp"

auto Counter::increment() -> void {
    curr++;
}

auto Counter::get() -> int {
    return curr;
}
