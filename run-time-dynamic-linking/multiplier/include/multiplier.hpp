#pragma once

namespace multiplier {

class Worker {
   public:
    Worker(int value);
    auto apply(int coefficient) -> void;
    auto get() -> int;

   private:
    int value;
};

}  // namespace multiplier

extern "C" auto example(int value) -> int;