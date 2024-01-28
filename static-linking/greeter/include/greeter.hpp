#pragma once

#include <string>

class Greeter {
   public:
    Greeter(std::string greeting);
    auto greet(std::string name) -> std::string;
    auto with_greeting(std::string greeting) -> Greeter;

   private:
    std::string greeting;
};