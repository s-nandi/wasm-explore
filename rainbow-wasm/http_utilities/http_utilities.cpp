#include "http_utilities.hpp"

#include <iostream>

auto http::upload(const std::vector<char>& bytes, std::string name) -> void {
    std::string builder;
    for (const auto& c : bytes) {
        builder += c;
        builder += "-";
    }
    builder.pop_back();

    std::cout << "Uploading " << builder << " to " << name << '\n';
}