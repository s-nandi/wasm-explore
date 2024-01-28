#pragma once

namespace adder {

class Worker {
   public:
    Worker(int value);
    auto apply(int delta) -> void;
    auto get() -> int;

   private:
    int value;
};

}  // namespace adder

extern "C" auto example(int value) -> int;