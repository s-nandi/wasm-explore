#include <dlfcn.h>

#include <exception>
#include <iostream>

#include "adder.hpp"
#include "multiplier.hpp"

using namespace std::literals;

int main(int argc, char** argv) {
    if (argc < 3) {
        std::cout << "invalid invokation" << '\n';
        std::cout << "usage: " << argv[0] << " <library> <number>" << '\n';
        return 1;
    }

    auto library_name = argv[1];
    auto value = atoi(argv[2]);
    std::cout << "input: " << library_name << ": " << value << '\n';

    auto library_handle = dlopen(library_name, RTLD_LAZY);
    if (library_handle == nullptr) {
        throw std::runtime_error("Dlopen failed to open: "s + library_name);
    }
    using function_type = int (*)(int);
    auto function_name = "example";
    function_type library_operation = (function_type)dlsym(library_handle, function_name);

    if (library_operation == nullptr) {
        throw std::runtime_error("Invalid function name "s + function_name + " in " + library_name);
    }

    auto result = library_operation(value);
    std::cout << "Result = " << result << '\n';
}