#include "greeter.hpp"

Greeter::Greeter(std::string greeting) : greeting{greeting} {}

auto Greeter::greet(std::string name) -> std::string {
    return greeting + " " + name;
}

auto Greeter::with_greeting(std::string greeting) -> Greeter {
    auto other = *this;
    other.greeting = greeting;
    return other;
}