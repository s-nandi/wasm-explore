#pragma once

#include <array>
#include <string>
#include <vector>

namespace scene_utils {

struct Camera {
    std::string name;
    std::array<int, 3> position;
};

class Asset {
   public:
    Asset(std::string name);
    auto name() const -> std::string;
    auto cameras() const -> std::vector<Camera>;

   private:
    std::string name_;
    std::vector<Camera> cameras_;
};

class Image {
   public:
    Image(std::string name);
    auto name() const -> std::string;
    auto bytes() -> std::vector<char>;

   private:
    std::string name_;
};

auto load(const std::string& path) -> Asset;

auto render(const Asset& asset, const Camera& camera) -> Image;

}  // namespace scene_utils

namespace scene_utils::detail {

auto make_camera(int i) -> Camera;

}