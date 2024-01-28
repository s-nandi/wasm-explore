#include <iostream>

#include "a.hpp"
#include "b.hpp"

int main() {
    std::cout << "Main" << '\n';
    projecta::foo();
    projectb::foo();
}