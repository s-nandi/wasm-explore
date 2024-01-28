#pragma once

class Counter {
   public:
    auto increment() -> void;
    auto get() -> int;

   private:
    int curr{0};
};