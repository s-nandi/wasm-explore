#include <iostream>

#include "lib.hpp"

int main() {
    auto greeter_1 = Greeter{"Hello"};
    auto greeting_1 = greeter_1.greet("Bob");
    auto greeter_2 = greeter_1.with_greeting("Good morning");
    auto greeting_2 = greeter_2.greet("Alice");
    std::cout << greeting_1 << '\n';
    std::cout << greeting_2 << '\n';

    auto counter = Counter{};
    for (int i = 0; i < 5; i++) {
        std::cout << "Counter = " << counter.get() << '\n';
        counter.increment();
    }
}