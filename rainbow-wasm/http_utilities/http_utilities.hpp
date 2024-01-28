#pragma once

#include <string>
#include <vector>

namespace http {

auto upload(const std::vector<char>& bytes, std::string name) -> void;

}